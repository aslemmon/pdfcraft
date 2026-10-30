# PdfCraft – Personal Customizations

This document tracks personal enhancements and modifications to PdfCraft.

The intention is to maintain a customised version of PdfCraft while keeping it as compatible as possible with the original upstream project (https://github.com/storytold/pdfcraft).

## Development Strategy

Customisations are developed independently in feature branches and consolidated into a dedicated personal-build branch.

The original project's main branch remains as close as possible to upstream.

## Branch Structure

| Branch | Purpose |
|---|---|
| `main` | Clean copy of the original upstream project |
| `feature/triple-click-highlight` | Triple-click text highlighting |
| `feature/deselect-after-highlight` | Automatically deselect after highlighting |
| `feature/custom-highlight-colors` | Save and reuse custom highlight colors |
| `feature/hide-toolbars` | Show/hide application toolbars |
| `feature/highlight-keyboard-shortcuts` | Keyboard shortcuts for highlighting and color selection |
| `feature/multiple-windows` | Open documents in separate application windows |
| `feature/overlapping-text-highlights` | Prevent same-color overlapping highlights from becoming darker |
| `personal-build` | Consolidated version containing all completed customisations |

## Feature Roadmap

### 1. Triple-click Highlights

**Branch:** `feature/triple-click-highlight`

**Objective:** Enable triple-clicking to select and highlight a line of text, while retaining the existing double-click word selection behavior.

**Status:** Implemented and committed.

**Commit:** `d56f0ef8`

**Verified behavior:**
- Double-click highlights a word.
- Triple-click highlights a line.
- Additional clicks do not create extra highlights.
- Existing text selection behavior remains intact.

### 2. Deselect After Highlight

**Branch:** `feature/deselect-after-highlight`

**Objective:** Automatically clear the active text selection after a highlight annotation has been created.

**Desired behavior:**
- Create the highlight annotation normally.
- Clear the active text selection immediately afterward.
- Keep the current highlighting tool active so another selection can be made.
- Do not remove or otherwise affect the newly created annotation.

**Priority:** High – next feature to implement.

### 3. Save and Store Custom Highlight Colors

**Branch:** `feature/custom-highlight-colors`

**Objective:** Allow users to define custom highlight colors and save them for reuse across documents and application sessions.

**Status:** Planned.

### 4. Hide Toolbars

**Branch:** `feature/hide-toolbars`

**Objective:** Provide the ability to hide and restore application toolbars to create a cleaner document viewing and reading experience.

**Status:** Planned.

### 5. Keyboard Shortcuts for Highlights and Colors

**Branch:** `feature/highlight-keyboard-shortcuts`

**Objective:** Introduce keyboard shortcuts for common highlighting actions and quick selection of highlight colors.

**Status:** Planned.

### 6. Open Files in Multiple Windows

**Branch:** `feature/multiple-windows`

**Objective:** Allow multiple PDF documents to be opened in separate application windows rather than requiring all documents to share a single window.

**Status:** Planned.

### 7. Prevent Overlapping Highlights from Becoming Darker

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
| 2026-10-29 | Created customisation roadmap | `main` | Initial development plan established |
| 2026-10-29 | Implemented triple-click highlighting | `feature/triple-click-highlight` | Committed as `d56f0ef8`; automated and manual testing completed |
| 2026-10-29 | Prioritised deselection after highlighting | `feature/deselect-after-highlight` | Identified as next implementation task |
| 2026-10-29 | Added overlapping-highlight improvement | `feature/overlapping-text-highlights` | Added to roadmap as a lower-priority improvement |

---

This document is maintained alongside the source code and updated as customisations are implemented.
