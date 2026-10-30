PdfCraft – Personal Customizations

This document tracks personal enhancements and modifications to PdfCraft.

The intention is to maintain a customised version of PdfCraft while keeping it as compatible as possible with the original upstream project (https://github.com/storytold/pdfcraft).

Development Strategy

Customisations are developed independently in feature branches and consolidated into a dedicated personal-build branch.

The original project's main branch remains as close as possible to upstream.

Branch Structure
Branch	Purpose
main	Clean copy of the original upstream project
feature/triple-click-highlight	Triple-click text highlighting
feature/deselect-after-highlight	Automatically deselect after highlighting
feature/custom-highlight-colors	Save and reuse custom highlight colors
feature/hide-toolbars	Show/hide application toolbars
feature/highlight-keyboard-shortcuts	Keyboard shortcuts for highlighting and color selection
feature/multiple-windows	Open documents in separate application windows
personal-build	Consolidated version containing all completed customisations
Feature Roadmap
1. Triple-click Highlights

Branch: feature/triple-click-highlight

Objective: Enable triple-clicking to select and highlight a word, line, or other appropriate text selection.

Status: Planned

2. Deselect After Highlight

Branch: feature/deselect-after-highlight

Objective: Automatically clear the active text selection after a highlight annotation has been created.

Status: Planned

3. Save and Store Custom Highlight Colors

Branch: feature/custom-highlight-colors

Objective: Allow users to define custom highlight colors and save them for reuse across documents and application sessions.

Status: Planned

4. Hide Toolbars

Branch: feature/hide-toolbars

Objective: Provide the ability to hide and restore application toolbars to create a cleaner document viewing and reading experience.

Status: Planned

5. Keyboard Shortcuts for Highlights and Colors

Branch: feature/highlight-keyboard-shortcuts

Objective: Introduce keyboard shortcuts for common highlighting actions and quick selection of highlight colors.

Status: Planned

6. Open Files in Multiple Windows

Branch: feature/multiple-windows

Objective: Allow multiple PDF documents to be opened in separate application windows rather than requiring all documents to share a single window.

Status: Planned

Upstream Compatibility

The project should remain easy to update as the original developers continue development.

Guidelines:

Keep individual customisations isolated in feature branches.
Avoid unnecessary changes to existing functionality.
Avoid unnecessary dependency or Rust toolchain upgrades.
Keep main synchronised with upstream.
Regularly integrate upstream changes into customisation branches.
Test consolidated changes before building a new personal release.
Consider submitting generally useful improvements as upstream pull requests.
Change History
Date	Change	Branch	Notes
2026-10-29	Created customisation roadmap	main	Initial development plan established

This document is maintained alongside the source code and updated as customisations are implemented.
