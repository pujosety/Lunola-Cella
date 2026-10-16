# Known Issues

This document lists known limitations and issues in Lunola Cella at the
time of the v1.0.0 release (2026-10-16).

## Build Requirements

| Requirement | Status | Notes |
|---|---|---|
| **Rust version** | `rustc 1.95+` required | The project uses Rust edition 2024 with `resolver = "3"`. Verified with rustc 1.99.1 stable. |
| **wasm32 target** | Required for web builds | `rustup target add wasm32-unknown-unknown` |
| **trunk** | Required for web builds | Install from [trunkrs.dev](https://trunkrs.dev) |

## Verification Status

### Automated Tests

**`cargo check -p gridcraft-ui-egui`**: ✅ PASS (rustc 1.99.1, 2 warnings, 0 errors)

**Full `cargo test --workspace`**: NOT RUN — requires all git dependencies (krilla/craft-libs) to be fetchable. Run:

```sh
cargo test --workspace
cargo xtask ci
```

before relying on any build.

### GUI Verification

**No GUI verification was performed** as part of the v1.0.0 delivery. The
desktop application has not been launched or exercised interactively. All UI
changes are based on code review and asset replacement only.

## Known Limitations

### File Format

- **XLSX fidelity**: Not verified against a real-world corpus. Excel may show
  repair prompts on some files produced by Lunola Cella.
- **Large workbooks**: 100 MB+ XLSX files may not open (#175). The CSV export
  cap is 512 MiB.
- **`.xls` (BIFF8)**: Import is supported but formatting and objects are not
  preserved. No `.xls` export.
- **XLSB**: Worksheet data can be imported using saved values; formulas are
  lost. See [docs/xlsb-import.md](docs/xlsb-import.md).
- **ODS**: Worksheet data can be imported using saved values; formulas are
  lost. See [docs/ods-import.md](docs/ods-import.md).
- **Password-protected workbooks**: Can be opened (Agile and Standard
  encryption), but cannot be saved with a password.

### Charts

- Chart types and options may not round-trip perfectly through XLSX save and
  reopen.
- Chart depth (axes, trendlines, error bars, 3D charts) is incomplete.
- See [docs/chart-parity.md](docs/chart-parity.md).

### Functions

- 513 of Excel's 523 worksheet functions are implemented (98.1%).
- 10 functions remain unimplemented.
- See [docs/function-parity.md](docs/function-parity.md).

### Localization

- Ribbon translated into 12 languages (en, ja, zh, ko, pt-BR, ru, de, cs, fr, sr, id).
- Dialogs, menus, and messages are English-only.
- Formulas and number formats are en-US only.
- Arabic and Devanagari text shaping is not implemented.
- See [docs/localization-parity.md](docs/localization-parity.md).

### Text Rendering

- CJK, Thai, and Hebrew text rely on system font fallback.
- Arabic and Devanagari shaping is not implemented (#142).

### Platforms

- **iOS/Android**: Not supported.
- **Web**: WebAssembly build is functional but browser printing and PDF export
  are the only output paths.
- **Linux/BSD**: Supported but less tested than macOS and Windows.

### Performance

- No multi-threaded recalculation (single-threaded dependency graph).
- Large workbooks may be slow to open and recalculate.
- See [docs/roadmap.md](docs/roadmap.md) for beta gates.

### Collaboration

- No real-time collaboration.
- No cloud save or sync.
- No threaded comment UI (XLSX round-trip works, but the UI is basic).

### AI Features

- Agent control via MCP, CLI, and control channel is strong.
- No in-app AI assistant.
- No formula generation from natural language.

### Hardware

- No GPU-accelerated rendering (CPU-only).
- No pen/inking support on Windows tablets.
- No multi-window support (single window per process).
- See [docs/hardware-parity.md](docs/hardware-parity.md).

### UI/UX

- Ribbon key tips are initial support only; full mouse-free navigation is
  incomplete.
- Some dialogs may differ from Excel in layout and behavior.
- Print preview is functional but may differ from Excel's.
- See [docs/ui-parity.md](docs/ui-parity.md).

## Reporting Issues

Please report issues at
[https://github.com/pujosety/Lunola-Cella/issues](https://github.com/pujosety/Lunola-Cella/issues).

Include:
1. Your operating system and version.
2. Lunola Cella version (from About dialog).
3. Steps to reproduce.
4. The `.xlsx` file if applicable.
5. Log file from `logs/gridcraft.log` (or `gridcraft.1.log`).
