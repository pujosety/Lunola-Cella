# Changelog

All notable changes to Lunola Cella are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-10-16

### Rebranding: GridCraft → Lunola Cella

This release marks the complete rebranding of GridCraft to **Lunola Cella**,
a member of the Lunola Suite of open-source creative tools. The project is now
positioned as "Every Cell Counts."

### Changed

- **Product name**: All user-facing strings, window titles, and packaging now use
  "Lunola Cella" instead of "GridCraft."
- **Accent color**: The UI accent color is now `#35B78C` (Lunola green), replacing
  the previous GridCraft branding color.
- **Tagline**: "Every Cell Counts." is displayed in the About dialog and
  startup splash.
- **App icons**: New Lunola Cella icon set (SVG, ICO, ICNS, hicolor PNGs) derived
  from the Lunola brand assets.
- **Logo and splash**: New `docs/brand/` directory with Lunola Cella logos
  (light/dark), marks, and a startup splash image.
- **macOS packaging**: DMG background, Info.plist, entitlements, and package
  scripts updated for Lunola Cella.
- **Windows packaging**: WiX installer, preview dialogs, signing scripts, and
  app-build config updated for Lunola Cella.
- **Linux packaging**: Desktop entry, metainfo, MIME types, Flatpak, nfpm, and
  package scripts updated for Lunola Cella.
- **FreeBSD packaging**: Package script updated for Lunola Cella.
- **Web app**: `index.html`, host module, and web entry point updated for
  Lunola Cella.
- **README**: Rewritten with Lunola Cella branding, new screenshots section,
  and links to all documentation.
- **NOTICE and licenses**: Copyright updated to "Lunola Team and the Lunola
  Cella contributors."
- **Attribution**: Asset table updated with all new brand files and Lunola
  Cella-specific entries.
- **Localization**: Indonesian (`id-ID`) locale added with 2,948 lines of
  translated ribbon strings.
- **CLI**: `gridcraft-cli` help text, MCP server descriptions, and command
  output updated for Lunola Cella.
- **Engine**: Command names, log messages, and error strings updated for Lunola
  Cella.
- **UI (egui)**: Theme, ribbon, dialogs, backstage, formula bar, and i18n
  modules updated for Lunola Cella branding.
- **XLSX**: Import/export module comments updated for Lunola Cella.
- **MCP server**: Tool descriptions and server metadata updated for Lunola Cella.

### Added

- **Brand assets**: `docs/brand/` directory with Lunola Cella logos, marks,
  and brand license (`LICENSE-brand.txt`).
- **App icons**: New icon set in `assets/app-icon/` (SVG, PNG at all standard
  sizes, ICO, ICNS).
- **Brand icons**: `assets/brand/gridcraft-icon.svg`, `gridcraft-logo.svg`,
  `gridcraft-logo.png`, `gridcraft-splash.png`.
- **Indonesian locale**: `crates/ui-egui/src/i18n/id.rs` with full ribbon
  translation.
- **Lunola Suite integration**: README section listing all Lunola apps with
  icons and links.
- **Deliverable documentation**: `CHANGELOG.md`, `KNOWN_ISSUES.md`,
  `DELIVERY.md`, and updated `ROADMAP.md`.

### Deprecated

- None.

### Removed

- Old GridCraft logo and splash assets (replaced by Lunola Cella brand files).

### Fixed

- All icon paths updated to reference new Lunola Cella brand assets correctly.

### Security

- No security-related changes in this release.

---

## [0.7.0] - 2026-10-09

### Added

- Rows auto-fit oversized fonts on open and whole-row/column formatting (#833).
- Number format parsing is linear in format code length (#810).
- Fraction denominator crash with 65,536 zeros fixed; format codes capped at
  255 characters like Excel (#807).
- Cell hyperlinks restricted to http, https, and mailto with confirmation
  dialog (#777).
- Multi-area row/column delete in one undo step (#758).
- XLSX chart sheets preserved on save (#727).
- CSV/TSV export size cap at 512 MiB (#528).
- PDF writer hardening for text width, dash arrays, stroke widths, and
  non-finite values (#812).
- PDF export writes document title and author as UTF-16BE for non-ASCII
  text (#813).
- `TEXT()` and chart labels parse number format codes once per call (#806).
- SUBTOTAL(109) recalculates after row height changes and ungroup (#758).
- Page Layout view: drag margin edges on rulers (#792).
- Forecast Sheet gets Forecast Start (#785).
- Ink strokes save as DrawingML paths and reopen as editable (#551).
- Selection outline stays visible at last row with Ctrl+Shift+Down (#687).
- Desktop printing with system printer discovery, copies, duplex, grayscale,
  and spooler error display.
- Command-sequence fuzzing with undo/redo, XLSX round-trip, and recalc
  verification.
- Password-protected workbook decryption (Agile and Standard encryption)
  (#11).
- Text box formatting preservation (run size, bold, italic, typeface, text
  colour, outline, alignment, vertical anchor) (#377).
- macOS File > Properties opens File > Info (#343).
- Windows: DirectX 12 shaders compile with FXC, no external dxcompiler.dll
  needed (#339, #354).
- Merged cells draw full outline (#376).
- Shrink to fit for too-wide text and numbers (#375).
- Shift+Space and Ctrl+Space select row/column without starting edit (#338).
- Workbooks reopen at saved scroll position (#378).
- Protect Sheet passwords use Excel-compatible hashes and round-trip in XLSX
  (#300).
- Protect Workbook dialog with optional password (#345).
- Formula bar, name box, and dialogs fall back to cell font (#349).
- Headers and footers: first-page and odd/even variants (#344).
- Legacy Excel 97-2003 `.xls` (BIFF8) import (#121).
- XLSX pass-through for custom XML, external links, connections, query
  tables, printer settings, form/ActiveX controls, and OLE objects (#330,
  #324).
- Data > Forecast > Forecast Sheet (#322).
- Startup font validation to prevent panics (#226).
- Ctrl+drag on fill handle swaps copy and extend (#264).
- ODDFPRICE, ODDFYIELD, ODDLPRICE, ODDLYIELD financial functions (#275).
- PERCENTOF and EUROCONVERT functions (#279).
- BAHTTEXT function (#291).
- TRIMRANGE function (#296).
- Keyboard deletion for selected charts, pictures, and shapes (#213).
- German ribbon translation (266 labels).
- View > Interface size: 75-200% scale (#230).
- View > Formula Bar and Zoom 100% reachable from CLI and MCP (#289).
- Freeze panes by dragging freeze handles (#195).
- XLSX threaded comments round-trip (#300).
- Cell checkboxes with XLSX round-trip as Excel cell controls (#196).
- Review > Allow Edit Ranges with password (#307).
- PivotTable calculated fields with XLSX round-trip (#304).
- PivotCharts (#311).
- FORECAST.ETS, FORECAST.ETS.CONFIDENCE, FORECAST.ETS.SEASONALITY,
  FORECAST.ETS.STAT functions (#314).
- File tab Backstage view (#174).

### Changed

- Ribbon translated into Japanese, Simplified Chinese, Korean, Brazilian
  Portuguese, and Russian with system-language default (#31, #104, #82).
- CJK, Thai, and Hebrew font fallback (#38).
- Progress docs restructured to craftrules standard.
- Full re-measurement against Excel for Mac 16.113.4.

### Fixed

- Spill range operator `A1#` parsing.
- Sheet-scoped names resolution.
- Protection refuses locked-cell edits.
- Insert/Delete Cells adjust references.
- DATE(1900,2,29) Excel compatibility.
- SUMIF sizing for larger ranges.
- ROUND at 15 significant digits.
- `_xlpm.` LAMBDA parameter prefix in XLSX.
- Vertical scrollbar behavior.
- System appearance detection.
- Border previews in Format Cells dialog.
