# Changelog

All notable changes to **Rust Laboratory: Signal Zero** are documented here.

## [0.1.0] — 2026-08-18

### Added

- A native desktop interface for Windows built with Rust and `eframe/egui`.
- A complete playable mystery, **Signal Zero**, with branching choices, evidence gating and three distinct endings.
- A facility map, evidence ledger, progress tracker and persistent system log.
- Local, versioned JSON save files with atomic writes and local accessibility settings.
- Keyboard action selection, scalable text, high-contrast mode and optional action hints.
- Content validation that checks story nodes, destinations, evidence and requirements when the game starts.
- Unit tests for the initial state, locked-choice logic and the ethical resolution path.
- Cross-compilation configuration and a Windows x86_64 release build.
- Product architecture and global commercial roadmap documentation.

### Changed

- Replaced the original command-line room demo with a data-driven desktop game architecture.
- Rewrote the README for Windows users, contributors and release verification.

### Privacy

- The application runs offline by default and does not include an account system, cloud save or analytics SDK.
