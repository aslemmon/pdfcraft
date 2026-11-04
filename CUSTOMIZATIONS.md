# PdfCraft – Personal Customizations

This document tracks personal enhancements and modifications to PdfCraft.

The intention is to maintain a customized version of PdfCraft while keeping it as compatible as possible with the original upstream project ([https://github.com/storytold/pdfcraft](<https://github.com/storytold/pdfcraft>)).

## Development Strategy

Customizations are developed independently in feature branches and consolidated into a dedicated `personal-build` branch.

The original project's `main` branch remains as close as possible to upstream.

Completed features are merged into `personal-build` after testing. Individual feature branches are retained as reference points for future development.

## Branch Structure

| Branch | Purpose | Status |
| --- | --- | --- |
| `main` | Clean copy of the original upstream project | Maintained |
| `feature/triple-click-highlight` | Triple-click text highlighting | Complete |
| `feature/deselect-after-highlight` | Automatically deselect after highlighting | Complete |
| `feature/custom-highlight-colors` | Save and reuse custom highlight colors; annotation shortcuts | Complete |
| `feature/hide-toolbars` | Show/hide application toolbars | Complete |
| `feature/highlight-keyboard-shortcuts` | Originally planned keyboard shortcuts | Superseded |
| `feature/multiple-windows` | Open documents in separate application windows | Planned |
| `feature/overlapping-text-highlights` | Prevent overlapping highlights from blending or becoming darker | Complete |
| `personal-build` | Consolidated version containing completed customizations | Active |

## Feature Roadmap

### 1\. Triple-click Highlights

**Branch:** `feature/triple-click-highlight`

**Objective:** Enable triple-clicking to select and highlight a line of text, while retaining the existing double-click word selection behavior.

**Status:** Complete and merged into `personal-build`.

**Commit:** `d56f0ef8`

**Verified behavior:**

- Double-click highlights a word.
- Triple-click highlights a line.
- Additional clicks do not create extra highlights.
- Existing text selection behavior remains intact.

### 2\. Deselect After Highlight

**Branch:** `feature/deselect-after-highlight`

**Objective:** Keep newly created highlight annotations deselected, rather than automatically selecting them in the Comments panel.

**Implementation status:** Complete.

**Tests:** Updated integration tests verify that text and area highlights remain deselected and do not automatically open the note editor.

**Personal-build integration:** Confirmed.

**Integration commit:** `b0b67f60`.

### 3\. Custom Highlight Colors and Annotation Keyboard Shortcuts

**Branch:** `feature/custom-highlight-colors`

**Objective:** Improve annotation efficiency by allowing users to save custom highlight colors and quickly access annotation tools and saved colors using keyboard shortcuts.

**Implemented behavior:**

- Allow users to create custom highlight colors.
- Save favorite colors persistently across application sessions.
- Retain the existing standard highlight colors.
- Remember the last-used color.
- Provide keyboard shortcuts for activating the highlighter and other commonly used annotation tools.
- Provide keyboard shortcuts for selecting saved highlight colors.
- Provide a keyboard shortcut for returning to the Select tool.
- Integrate shortcuts with the existing application keyboard handling.

**Implementation status:** Complete.

**Commits:**

- `af672818` – Add persistent custom highlight colour pins.
- `6c73bc27` – Add annotation keyboard shortcuts.

**Personal-build integration:** Confirmed.

**Integration commit:** `21c8c5f0`.

**Testing:** Build completed successfully, UI test suite and manual feature testing completed.

### 4\. Hide Toolbars

**Branch:** `feature/hide-toolbars`

**Objective:** Provide the ability to hide and restore application toolbars to create a cleaner document viewing and reading experience.

**Implemented behavior:**

- Added preferences to control the visibility of application toolbar elements.
- Added independent visibility controls for the main toolbar and right navigation toolbar.
- Added visibility controls for individual title-bar buttons, including Preferences, display theme, keyboard shortcuts and Discord.
- Preserved window controls and their platform-specific layout.
- Adjusted toolbar width calculations to account for buttons that are actually visible.
- Improved tab sizing and scrolling behavior when toolbar space is limited.
- Kept the Open button visible after the scrolling document tabs.
- Preserved GNOME on Wayland window dragging and double-click maximize behavior.
- Maintained the native macOS menu bar behavior.
- Preserved the page-size hover indicator and its existing positioning.

**Implementation status:** Complete.

**Branch:** `feature/hide-toolbars`.

**Testing:** `cargo build` completed successfully after implementation and cleanup.

Two compiler warnings introduced by the page-size hint changes were corrected:

- Removed the unused page binding in `canvas/bars.rs`.
- Explicitly ignored the unused `units` field in the `PageEnv` destructuring in `canvas.rs`.

The remaining Cargo warnings concern unused dependency patches for optional egui packages and are unrelated to the toolbar implementation.

**Personal-build integration:** Ready for integration following the feature-branch merge.

### 5\. Open Files in Multiple Windows

**Branch:** `feature/multiple-windows`

**Objective:** Allow multiple PDF documents to be opened in separate application windows rather than requiring all documents to share a single window.

**Status:** Planned.

### 6\. Prevent Overlapping Highlights from Becoming Darker

**Branch:** `feature/overlapping-text-highlights`

**Objective:** Make overlapping highlights behave consistently with macOS Preview, with the newest highlight replacing the color of earlier highlights in the overlapping region.

**Desired and implemented behavior:**

- Applying the same highlight color repeatedly does not progressively darken the overlapping area.
- Applying a different highlight color replaces the previous color in the selected region.
- The newest highlight takes precedence wherever highlight regions overlap.
- Existing highlight colors remain visible outside the overlapping region.
- Highlighting previously unhighlighted text continues to work normally.
- Text remains readable through the highlights.
- Other annotation types, particularly Underline and StrikeOut, retain their existing rendering behavior.

**Implementation approach:**

Rather than modifying annotation geometry, merging annotations, or changing their opacity, the solution uses PDF transparency knockout groups during rendering.

All Highlight annotations are rendered together inside a non-isolated knockout group. The existing rendering infrastructure handles the necessary blend-mode passes, allowing newer highlights to replace earlier paints in their covered regions instead of blending with them.

The implementation is confined to:

`vendor/hayro-interpret/src/interpret/mod.rs`

The existing Multiply blend mode and individual annotation opacity settings are preserved.

**Important characteristics:**

- No changes to annotation creation or editing.
- No changes to `/QuadPoints` or original glyph geometry.
- No changes to annotation colors, opacity, or metadata.
- No new dependencies.
- Existing annotation data remains intact and compatible with standard PDF rendering.
- Highlights are grouped for rendering, so their stacking relationship with other annotation types may differ from their original interleaving in the PDF annotation array.

**Implementation status:** Complete.

**Testing:**

- Repeated same-color highlighting produces no additional darkening.
- Different-color highlighting replaces the previous color.
- Reverse color replacement works correctly.
- Partial overlaps preserve the original color outside the new selection.
- Black text remains readable.
- Underline and StrikeOut remain visible above highlights, regardless of application order.
- Undo and redo work correctly.
- Saving and reopening preserves the annotations and rendering behavior.
- Opening the resulting PDF in standard PDFCraft confirms that the original annotation data remains intact; the difference is in PDFCraft's rendering behavior.

**Remaining validation:** Skewed and rotated OCR text should be checked when suitable test documents are available. The implementation does not modify text geometry or annotation quadrilaterals.

**Personal-build integration:** Complete.

## Upstream Compatibility

The project should remain easy to update as the original developers continue development.

Guidelines:

- Keep individual customizations isolated in feature branches.
- Avoid unnecessary changes to existing functionality.
- Avoid unnecessary dependency or Rust toolchain upgrades.
- Keep `main` synchronized with upstream.
- Regularly integrate upstream changes into customization branches.
- Test consolidated changes before building a new personal release.
- Consider submitting generally useful improvements as upstream pull requests.

## Change History

| Date | Change | Branch | Notes |
| --- | --- | --- | --- |
| 2026-10-29 | Created customization roadmap | `personal-build` | Initial development plan established |
| 2026-10-29 | Implemented triple-click highlighting | `feature/triple-click-highlight` | Committed as `d56f0ef8`; automated and manual testing completed |
| 2026-10-29 | Implemented deselection after highlighting | `feature/deselect-after-highlight` | Committed as `b852f664`; integration testing completed |
| 2026-10-29 | Added overlapping-highlight improvement | `personal-build` | Added to roadmap as a lower-priority improvement |
| 2026-10-31 | Added persistent custom highlight color pins | `feature/custom-highlight-colors` | Committed as `af672818` |
| 2026-10-31 | Added annotation keyboard shortcuts | `feature/custom-highlight-colors` | Committed as `6c73bc27` |
| 2026-10-31 | Merged triple-click and deselect features | `personal-build` | Merge commit `b0b67f60` |
| 2026-10-31 | Merged custom highlight colors and annotation shortcuts | `personal-build` | Merge commit `21c8c5f0`; build and manual testing completed |
| 2026-11-03 | Implemented overlapping-highlight rendering improvement | `feature/overlapping-text-highlights` | Knockout rendering; no annotation-data or geometry changes |
| 2026-11-03 | Completed overlapping-highlight GUI testing and integration | `personal-build` | Color replacement, readability, undo/redo, save/reopen verified |
| 2026-11-03 | Implemented hide-toolbars customization | `feature/hide-toolbars` | Toolbar visibility preferences and responsive toolbar layout completed |
| 2026-11-03 | Fixed toolbar-related compiler warnings | `feature/hide-toolbars` | Removed unused page binding and ignored unused `units` field; build verified |

---

This document is maintained alongside the source code on `personal-build` and updated as customizations are implemented.
