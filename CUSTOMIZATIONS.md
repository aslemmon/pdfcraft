# PdfCraft – Personal Customizations

This document tracks personal enhancements and modifications to PdfCraft.

The intention is to maintain a customised version of PdfCraft while keeping it as compatible as possible with the original upstream project (https://github.com/storytold/pdfcraft).

## Development Strategy

Customisations are developed independently in feature branches and consolidated into a dedicated `personal-build` branch.

The original project's `main` branch remains as close as possible to upstream.

Completed features are merged into `personal-build` after testing. Individual feature branches are retained as reference points for future development.

## Branch Structure

| Branch | Purpose | Status |
|---|---|---|
| `main` | Clean copy of the original upstream project | Maintained |
| `feature/triple-click-highlight` | Triple-click text highlighting | Complete |
| `feature/deselect-after-highlight` | Automatically deselect after highlighting | Complete |
| `feature/custom-highlight-colors` | Save and reuse custom highlight colors; annotation shortcuts | Complete |
| `feature/hide-toolbars` | Show/hide application toolbars | Planned |
| `feature/highlight-keyboard-shortcuts` | Originally planned keyboard shortcuts | Superseded |
| `feature/multiple-windows` | Open documents in separate application windows | Planned |
| `feature/overlapping-text-highlights` | Prevent same-color overlapping highlights from becoming darker | Planned |
| `personal-build` | Consolidated version containing completed customisations | Active |

## Feature Roadmap

### 1. Triple-click Highlights

**Branch:** `feature/triple-click-highlight`

**Objective:** Enable triple-clicking to select and highlight a line of text, while retaining the existing double-click word selection behavior.

**Status:** Complete and merged into `personal-build`.

**Commit:** `d56f0ef8`

**Verified behavior:**
- Double-click highlights a word.
- Triple-click highlights a line.
- Additional clicks do not create extra highlights.
- Existing text selection behavior remains intact.

### 2. Deselect After Highlight

**Branch:** `feature/deselect-after-highlight`

**Objective:** Keep newly created highlight annotations deselected, rather than automatically selecting them in the Comments panel.

**Implementation status:** Complete.

**Tests:** Updated integration tests verify that text and area highlights remain deselected and do not automatically open the note editor.

**Personal-build integration:** Confirmed.

**Integration commit:** `b0b67f60`.

### 3. Custom Highlight Colors and Annotation Keyboard Shortcuts

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

### 4. Hide Toolbars

**Branch:** `feature/hide-toolbars`

**Objective:** Provide the ability to hide and restore application toolbars to create a cleaner document viewing and reading experience.

**Status:** Planned.

### 5. Open Files in Multiple Windows

**Branch:** `feature/multiple-windows`

**Objective:** Allow multiple PDF documents to be opened in separate application windows rather than requiring all documents to share a single window.

**Status:** Planned.

### 6. Prevent Overlapping Highlights from Becoming Darker

**Branch:** `feature/overlapping-text-highlights`

**Objective:** Make overlapping highlights of the same color behave consistently with macOS Preview.

**Desired behavior:**
- Applying the same highlight color over existing highlights should not progressively darken the overlapping area.
- Overlapping highlights of the same color should appear visually consistent with a single highlight.
- Highlighting previously unhighlighted text should continue to work normally.
- Different highlight colors should remain independently supported.

**Status:** Planned – lower priority.

**Implementation consideration:** Investigate how Preview handles overlapping annotations before deciding whether to merge annotations, suppress duplicates, or adjust rendering behavior.

## Upstream Compatibility

The project should remain easy to update as the original developers continue development.

Guidelines:

- Keep individual customisations isolated in feature branches.
- Avoid unnecessary changes to existing functionality.
- Avoid unnecessary dependency or Rust toolchain upgrades.
- Keep `main` synchronised with upstream.
- Regularly integrate upstream changes into customisation branches.
- Test consolidated changes before building a new personal release.
- Consider submitting generally useful improvements as upstream pull requests.

## Change History

| Date | Change | Branch | Notes |
|---|---|---|---|
| 2026-10-29 | Created customisation roadmap | `personal-build` | Initial development plan established |
| 2026-10-29 | Implemented triple-click highlighting | `feature/triple-click-highlight` | Committed as `d56f0ef8`; automated and manual testing completed |
| 2026-10-29 | Implemented deselection after highlighting | `feature/deselect-after-highlight` | Committed as `b852f664`; integration testing completed |
| 2026-10-29 | Added overlapping-highlight improvement | `personal-build` | Added to roadmap as a lower-priority improvement |
| 2026-10-31 | Added persistent custom highlight color pins | `feature/custom-highlight-colors` | Committed as `af672818` |
| 2026-10-31 | Added annotation keyboard shortcuts | `feature/custom-highlight-colors` | Committed as `6c73bc27` |
| 2026-10-31 | Merged triple-click and deselect features | `personal-build` | Merge commit `b0b67f60` |
| 2026-10-31 | Merged custom highlight colors and annotation shortcuts | `personal-build` | Merge commit `21c8c5f0`; build and manual testing completed |

---

This document is maintained alongside the source code on `personal-build` and updated as customisations are implemented.
