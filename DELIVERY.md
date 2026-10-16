# Delivery Summary

## Lunola Cella v1.0.0 — Rebranding Delivery

**Date:** 2026-10-16  
**Branch:** `brand/lunola-cella`  
**Previous version:** 0.7.0

---

## Overview

This delivery finalizes the rebranding of GridCraft to **Lunola Cella**, a
member of the Lunola Suite of open-source creative tools. The project is now
positioned with the tagline "Every Cell Counts." and the Lunola accent color
`#35B78C`.

The rebranding is comprehensive: it touches every crate in the workspace,
all packaging (Windows, macOS, Linux, FreeBSD, web), the CLI, the MCP server,
the UI theme, the localization system, and all documentation.

---

## Phases

### Phase 1: Foundation Rebrand (commit `271c408`)

**Scope:** UI strings and packaging metadata.

- Replaced all user-facing "GridCraft" strings with "Lunola Cella" across
  the egui UI crate, engine, and CLI.
- Updated window titles, About dialog, and menu text.
- Updated `Cargo.toml` workspace metadata (repository URL, package names).
- Updated all crate `Cargo.toml` files with new descriptions.
- Updated packaging metadata: Windows WiX, macOS Info.plist and entitlements,
  Linux desktop entry and metainfo, FreeBSD package script.

### Phase 2: Brand Asset Creation (commit `1e523f9`)

**Scope:** Complete branding sweep with new visual identity.

- Created `docs/brand/` directory with Lunola Cella logos (SVG, PNG),
  brand marks (light/dark/black), and brand license.
- Created `assets/brand/` directory with app icon SVG, logo SVG/PNG, and
  startup splash PNG.
- Replaced all app icons: SVG, ICO, ICNS, and hicolor PNGs at all standard
  sizes (16, 24, 32, 48, 64, 128, 256, 512, 1024).
- Updated macOS DMG background SVG and generation script.
- Updated Windows installer icons and preview dialogs.
- Updated Linux Flatpak icons and desktop entry.
- Updated `NOTICE`, `LICENSE-MIT`, and `ATTRIBUTION.md` with Lunola Cella
  copyright and asset entries.

### Phase 3: Icon Path Updates (commit `2c3e88c`)

**Scope:** All icon references updated to new brand assets.

- Updated `apps/gridcraft/build.rs` to embed the new icon.
- Updated `apps/gridcraft/src/main.rs`, `graphics.rs`, `logging.rs`,
  `native_menu.rs`, `print.rs`, and `apple_events.rs` with new icon paths
  and app name strings.
- Updated `apps/gridcraft-web/` (index.html, host.rs, main.rs, web.rs).
- Updated `apps/gridcraft-cli/` (main.rs, check.rs).
- Updated all crate `Cargo.toml` files.

### Phase 4: UI Modernization (commit `9cae271`)

**Scope:** Visual refresh with Lunola Cella branding.

- Updated `crates/ui-egui/src/theme.rs` with the `#35B78C` accent color
  and Lunola Cella visual identity.
- Updated `crates/ui-egui/src/lib.rs` with new app metadata.
- Updated `crates/ui-egui/src/ribbon.rs` with new branding.
- Updated `crates/ui-egui/src/dialogs.rs` with new branding.
- Updated `crates/ui-egui/src/backstage.rs` with new branding.
- Updated `crates/ui-egui/src/formula_bar.rs` with new branding.
- Updated `crates/ui-egui/src/grid.rs` with new branding.
- Updated `crates/ui-egui/src/panes.rs` with new branding.
- Updated `crates/ui-egui/src/icons.rs` with new branding.
- Updated `crates/ui-egui/src/i18n/i18n.rs` and all locale files.
- Updated `crates/ui-egui/src/keytips/` with new branding.
- Updated `crates/engine/` modules (view, format, analysis, review, solver,
  spelling, whatif, extra) with new branding.
- Updated `crates/xlsx/` modules (lib, read, write, passthrough, pivot,
  feature_bags) with new branding.
- Updated `crates/mcp/` modules (lib, server, tools, backend) with new
  branding.
- Updated `crates/model/` modules (lib, features, preserved, style,
  workbook) with new branding.
- Updated `crates/pdf/` with new branding.
- Updated `crates/calc/` with new branding.
- Updated `crates/chart/` with new branding.
- Updated `crates/core/` with new branding.
- Updated `crates/formula/` with new branding.
- Updated `crates/functions/` with new branding.
- Updated `crates/numfmt/` with new branding.

### Phase 5: Indonesian Locale (commit `34ccd96`)

**Scope:** New `id-ID` translation.

- Added `crates/ui-egui/src/i18n/id.rs` with 2,948 lines of translated
  ribbon strings for Indonesian.

### Phase 6: Lunola Ecosystem Integration (commit `8f9b8d9`)

**Scope:** Integration with the Lunola Suite.

- Updated `README.md` with Lunola Cella branding, screenshots, Lunola
  Suite links, and comprehensive documentation.
- Updated `ROADMAP.md` with Lunola Cella references.
- Updated `docs/` with Lunola Cella references across all files:
  architecture, cell-pictures, chart-parity, cli, contributors,
  control-protocol, decisions, file-format-parity, formula-input,
  function-parity, gaps, hardware-parity, localization-parity, mcp,
  ods-import, parity-checklist, release-playbook, releasing, roadmap,
  target-app-parity, ui-parity, xlsb-import.
- Updated `packaging/` scripts (Linux, macOS, Windows, FreeBSD, web).
- Updated `.github/workflows/` (flatpak-pages, freebsd, nix, release,
  windows-arm64).
- Updated `AGENTS.md` and `CLAUDE.md` with Lunola Cella references.
- Updated `contributors/contributors.json`.
- Added new tests: `filter_menu_enter.rs`, `malformed.rs` (xlsx),
  formula parser tests.

### Phase 7: Delivery Documentation (this commit)

**Scope:** Final delivery documentation.

- Created `CHANGELOG.md` with v1.0.0 entry and v0.7.0 history.
- Created `KNOWN_ISSUES.md` with build requirements, verification status,
  and known limitations.
- Created `DELIVERY.md` (this file).
- Updated `README.md` with changelog, known issues, and roadmap links.
- Updated `ROADMAP.md` with v1.0.0 release notes.

---

## Deliverables

| File | Description |
|---|---|
| `CHANGELOG.md` | Version history with v1.0.0 and v0.7.0 entries |
| `KNOWN_ISSUES.md` | Known limitations and verification status |
| `DELIVERY.md` | This delivery summary |
| `README.md` | Updated with Lunola Cella branding and doc links |
| `ROADMAP.md` | Updated with v1.0.0 release notes |
| `docs/brand/` | Lunola Cella brand assets (logos, marks, license) |
| `assets/brand/` | App icons and splash images |
| `assets/app-icon/` | New icon set at all standard sizes |

---

## Constraints Respected

- **No crate renames**: All crate names remain `gridcraft-*` (e.g.,
  `gridcraft-core`, `gridcraft-ui-egui`). The product name changed to Lunola
  Cella, but the Rust package names are unchanged.
- **No file format ID changes**: All file extensions (`.xlsx`, `.csv`,
  `.tsv`, `.xls`, `.xlsb`, `.ods`) and internal format identifiers are
  unchanged.
- **Honest documentation**: Known issues and limitations are documented
  truthfully, including the lack of automated test runs and GUI verification.

---

## Post-Delivery Checklist

- [ ] Run `cargo test --workspace` and verify all tests pass
- [ ] Run `cargo xtask ci` and verify all checks pass
- [ ] Launch the desktop application and verify the UI
- [ ] Open an existing `.xlsx` file and verify compatibility
- [ ] Verify the macOS DMG installs and launches
- [ ] Verify the Windows MSI installs and launches
- [ ] Verify the Linux AppImage/Flatpak runs
- [ ] Verify the web build serves and functions
- [ ] Verify the CLI tool works correctly
- [ ] Verify the MCP server starts and responds
