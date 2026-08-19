# Rust Laboratory: Signal Zero

> A compact, narrative-driven laboratory mystery for Windows, built in Rust.

**Rust Laboratory: Signal Zero** turns the original command-line experiment into a polished desktop game. Explore a dark research annex, collect evidence, unlock the truth behind ORPHEUS, and reach one of several outcomes. The game is offline by default: it needs no account and stores saves locally.

## What is in this release

| Area | Included |
|---|---|
| Narrative | A branching laboratory mystery with multiple endings, evidence, gated discoveries and replayable choices. |
| Desktop UI | Story panel, numbered actions, facility map, evidence ledger, case progress and system log. |
| Accessibility | Full action selection by keyboard, scalable text, high-contrast mode, optional action hints and local settings. |
| Reliability | Versioned JSON saves, atomic writes, validated story content and automated engine tests. |
| Privacy | No account, cloud save, analytics SDK or third-party tracking in the application. |

## Windows quick start

1. Download `rust-laboratory-signal-zero-windows-x86_64.zip` from the release assets.
2. Extract the archive to a folder you control.
3. Run `rust-laboratory-signal-zero.exe`.
4. Select **Open new investigation**.

The Windows package relies only on standard Windows components and should be run on a supported 64-bit Windows installation. Windows may display a SmartScreen notice for an unsigned independent application; do not bypass security prompts unless you obtained the archive from the official project release and verified its published SHA-256 checksum.

## Controls

| Action | Control |
|---|---|
| Select an available story action | `1` through `9` |
| Save the local case file | `S` |
| Change text scale, contrast or hints | **Settings** |
| Start another route after an ending | **Open a new investigation** |

## Build from source

The project requires a current stable Rust toolchain.

```bash
git clone https://github.com/Ali-Marandi/RustTextAdventure.git
cd RustTextAdventure
cargo test
cargo run --release
```

To build a 64-bit Windows executable on Windows:

```powershell
cargo build --release
```

For cross-compilation from Linux, install the Rust target and a MinGW-w64 linker, then run:

```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

## Content architecture

Narrative content lives in `assets/story_en.json`. The Rust engine validates every story node, destination, evidence item and evidence requirement before the application starts. This separates writing from interface code and makes the next episode, a localization workflow, and an internal authoring tool feasible without replacing the core engine.

## Repository layout

```text
assets/                 Story content
src/app.rs              Desktop interface and accessibility controls
src/content.rs          Content schema and validation
src/game.rs             Game state, choices and endings
src/persistence.rs      Local save and settings storage
docs/                   Product and commercial roadmap documents
.github/workflows/      Quality and Windows build automation
```

## Quality checks

```bash
cargo fmt --check
cargo test
cargo clippy -- -D warnings
```

## Commercial roadmap

The detailed global validation, monetization, technical, legal, launch and growth plan is available in [`docs/GLOBAL_COMMERCIAL_ROADMAP_FA.md`](docs/GLOBAL_COMMERCIAL_ROADMAP_FA.md). The product and architecture specification is in [`docs/PRODUCT_AND_ARCHITECTURE.md`](docs/PRODUCT_AND_ARCHITECTURE.md).

## License

This repository is distributed under the [MIT License](LICENSE). Before a commercial release, ensure all text, visual assets, audio, trademarks and third-party packages used in the product have appropriate commercial rights and attribution.
