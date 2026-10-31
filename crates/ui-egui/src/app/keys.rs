use super::*;

impl PdfCraftApp {
    pub(super) fn shortcuts(&mut self, ctx: &egui::Context) {
        use egui::Key;
        let pasted = self.paste_key_released(ctx);
        // "Save changes?" is modal: its keys are its own (⌘D is Don't save there, not Document
        // properties; Escape cancels it rather than clearing a selection), and nothing may run
        // underneath it. They are read here, before the canvas can consume them.
        if self.close_request.is_some() {
            // These dialogs are drawn after the save prompt and own the keyboard while open.
            // Don't let Escape or Return resolve unsaved work underneath one.
            let covered = self.pending_link.is_some()
                || self.pending_remote_link.is_some()
                || self.pending_full_screen.is_some()
                || self.updates.open
                || self.dialog.is_some()
                || self.unlock_prompt.is_some();
            if !covered && let Some(choice) = dialogs::save_prompt_key(ctx, &mut self.save_prompt_button) {
                self.resolve_close(ctx, choice);
            }
            return;
        }
        // The other dialogs and prompts are modal too (⌘W closed the file behind Preferences,
        // #870).
        if self.modal_open() {
            return;
        }
        if let Some(view) = self.active.and_then(|i| self.views.get_mut(i))
            && view.auto_scroll.escape(ctx)
        {
            return;
        }
        // The in-window menu bar holds the keys while one of its menus is open or it has the
        // keyboard (Alt, F10); a command shortcut leaves it and runs.
        craft_menu::bar::leave_for_shortcut(ctx);
        if !craft_menu::bar::is_open(ctx) && !craft_menu::bar::is_focused(ctx) {
            self.registry_shortcuts(ctx);
        }
        // (Esc closes a Combine file shown large first.)
        if self.full_screen && self.combine_tab.preview.is_none() && ctx.input(|i| i.key_pressed(Key::Escape)) {
            self.set_full_screen(ctx, false);
        }
        if let Some(i) = self.active {
            // Select all belongs to the document or page grid, unless a text field or
            // overlay owns the keyboard. Other canvas shortcuts keep their own handling.
            let pages_owns_keyboard =
                self.right == Some(RightPanel::Pages) && self.views[i].pages_focus_id.is_some_and(|id| ctx.memory(|m| m.has_focus(id)));
            if self.dialog.is_none()
                && !self.palette_open
                && (pages_owns_keyboard || !ctx.egui_wants_keyboard_input())
                && ctx.input_mut(|input| input.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, Key::A)))
            {
                if pages_owns_keyboard {
                    self.views[i].select_all_pages();
                } else if self.is_preparing() {
                    if let Some(doc) = self.session.get(self.views[i].id) {
                        prepare::select_all(&mut self.views[i], &doc.form);
                    }
                } else {
                    self.views[i].select_all();
                }
            }
            self.tool_keys(i, ctx);
            if self.quick_tool == QuickTool::EditText
                && let Some(doc) = self.session.get(self.views[i].id).filter(|d| d.allows_modification())
            {
                object_ui::nudge_keys(ctx, &mut self.views[i], doc);
                object_ui::delete_keys(ctx, &mut self.views[i]);
            } else if let Some(doc) = self.session.get(self.views[i].id).filter(|d| d.allows_annotation()) {
                comments::nudge_keys(ctx, &mut self.views[i], &doc.info);
            }
            // A selected field moves with the arrows too (Prepare a form, #2608), before the view
            // turns the page with them.
            if self.is_preparing()
                && let Some(doc) = self.session.get(self.views[i].id).filter(|d| d.allows_modification())
            {
                prepare::nudge_keys(ctx, &mut self.views[i], doc);
            }
            canvas::shortcuts(&mut self.views[i], ctx, self.scroll_speed);
            self.copy_key(i, ctx);
            self.paste_image_key(i, ctx, pasted);
            self.image_pick_keys(i, ctx);
        }
    }
}

impl PdfCraftApp {
    /// The quick tools' keys, as in Acrobat: V selects, H pans, and holding Space pans until it
    /// is released. Plain letters, so not while a text field, a form field, a dialog or the
    /// palette has the keyboard.
    fn tool_keys(&mut self, i: usize, ctx: &egui::Context) {
        use egui::Key;

        const PREFIX_TIMEOUT: f64 = 1.0;

        let free = self.dialog.is_none() && !self.palette_open && !ctx.egui_wants_keyboard_input() && self.views[i].forms.focus.is_none();

        let (plain, space, now) = ctx.input(|input| (input.modifiers.is_none(), input.key_down(Key::Space), input.time));

        if let Some(previous) = self.space_hand
            && !space
        {
            self.space_hand = None;
            if self.quick_tool == QuickTool::Hand {
                self.quick_tool = previous;
            }
        }

        if !free || !plain {
            self.annotation_prefix_at = None;
            return;
        }

        if self.annotation_prefix_at.is_some_and(|started| now - started > PREFIX_TIMEOUT) {
            self.annotation_prefix_at = None;
        }

        // Number keys select pinned custom colours for markup annotation tools.
        // Do not handle them while an A-prefix shortcut is in progress.
        if self.annotation_prefix_at.is_none()
            && let QuickTool::Comment(tool) = self.quick_tool
            && tool.markup().is_some()
        {
            let slot = ctx.input(|input| {
                [(Key::Num1, 0), (Key::Num2, 1), (Key::Num3, 2), (Key::Num4, 3), (Key::Num5, 4)]
                    .into_iter()
                    .find_map(|(key, index)| input.key_pressed(key).then_some((key, index)))
            });

            if let Some((key, index)) = slot
                && let Some(colour) = self.comment_prefs.pinned_custom_colours.get(index).copied()
            {
                self.comment_prefs.set_color(tool, colour);
                ctx.input_mut(|input| {
                    input.consume_key(egui::Modifiers::NONE, key);
                });
            }
        }

        if self.annotation_prefix_at.is_some() {
            let next = ctx.input(|input| {
                if input.key_pressed(Key::H) {
                    Some(comments::CommentTool::Highlight)
                } else if input.key_pressed(Key::U) {
                    Some(comments::CommentTool::Underline)
                } else if input.key_pressed(Key::S) {
                    Some(comments::CommentTool::StrikeOut)
                } else if input.key_pressed(Key::Q) {
                    Some(comments::CommentTool::Squiggly)
                } else {
                    None
                }
            });

            if let Some(tool) = next {
                ctx.input_mut(|input| {
                    input.consume_key(egui::Modifiers::NONE, Key::H);
                    input.consume_key(egui::Modifiers::NONE, Key::U);
                    input.consume_key(egui::Modifiers::NONE, Key::S);
                    input.consume_key(egui::Modifiers::NONE, Key::Q);
                });
                self.annotation_prefix_at = None;
                self.quick_tool = QuickTool::Comment(tool);
                self.space_hand = None;
                self.comment_prefs.group_tool[tool.group()] = tool;
                return;
            }

            let cancel_keys = ctx.input(|input| {
                input
                    .events
                    .iter()
                    .filter_map(|event| match event {
                        egui::Event::Key { key, pressed: true, modifiers, .. } => Some((*key, *modifiers)),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            });

            if !cancel_keys.is_empty() {
                // Consume the invalid continuation so it cannot trigger another shortcut.
                ctx.input_mut(|input| {
                    for (key, modifiers) in cancel_keys {
                        input.consume_key(modifiers, key);
                    }
                });
                self.annotation_prefix_at = None;
                return;
            }
        }

        if ctx.input(|input| input.key_pressed(Key::A)) {
            ctx.input_mut(|input| {
                input.consume_key(egui::Modifiers::NONE, Key::A);
            });
            self.annotation_prefix_at = Some(now);
            return;
        }

        if ctx.input(|input| input.key_pressed(Key::V)) {
            self.quick_tool = QuickTool::Select;
            self.space_hand = None;
        }

        if ctx.input(|input| input.key_pressed(Key::H)) {
            self.quick_tool = QuickTool::Hand;
            self.space_hand = None;
        }

        if space && self.space_hand.is_none() && self.quick_tool != QuickTool::Hand {
            self.space_hand = Some(self.quick_tool);
            self.quick_tool = QuickTool::Hand;
        }
    }
}
