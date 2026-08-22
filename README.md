<div align="center">

# ⚡ THE RUST LABORATORY

**A Commercial-Grade Text Adventure Game**

Built with Rust + Tauri | Beautiful Cyberpunk GUI

[![GitHub Release](https://img.shields.io/github/v/release/Ali-Marandi/RustTextAdventure?style=flat-square)](https://github.com/Ali-Marandi/RustTextAdventure/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-2.0.0-orange.svg)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)

</div>

---

## 🎮 Overview

The Rust Laboratory is a fully-featured text-based adventure game with a stunning cyberpunk-themed GUI. You play as Dr. Alex Chen, an investigative journalist exploring the abandoned Prometheus Research Facility to uncover its secrets.

### ✨ Key Features

- **14+ Interconnected Rooms** - Each with unique descriptions, puzzles, and secrets
- **20+ Collectible Items** - With detailed lore and specific uses
- **6 Challenging Puzzles** - Code-breaking, item combination, repair challenges
- **13 Achievements** - Track your progress and unlock rewards
- **Beautiful Cyberpunk GUI** - Dark theme with animations, minimap, and particle effects
- **Save/Load System** - 3 save slots with auto-save support
- **Interactive Minimap** - Real-time map showing explored areas
- **Smart Autocomplete** - Context-aware command suggestions
- **Command History** - Arrow key navigation through previous commands

### 🌟 Screenshots

| Splash Screen | Game Interface |
|:---:|:---:|
| Particle-animated title screen with neon glow | Three-panel layout: stats, game, inventory |

---

## 💻 Installation

### Download (Recommended)

Download the latest release from [GitHub Releases](https://github.com/Ali-Marandi/RustTextAdventure/releases):

1. Download `RustTextAdventure_x64-setup.exe` (or `.msi`)
2. Run the installer
3. Launch "The Rust Laboratory" from your desktop

### Build from Source

**Prerequisites:** Rust 1.70+, Node.js 18+, Tauri CLI

```bash
git clone https://github.com/Ali-Marandi/RustTextAdventure.git
cd RustTextAdventure
cd src-tauri
cargo tauri build
```

---

## 🎮 How to Play

### Controls

| Input | Action |
|:---|:---|
| Type command + Enter | Execute command |
| Arrow Up/Down | Navigate command history |
| Tab | Accept autocomplete suggestion |
| Escape | Close modal/dialog |
| Click exit buttons | Quick navigation |
| Click items | Interact with items |

### Commands

| Command | Description |
|:---|:---|
| `go [direction]` | Move in a direction (north/south/east/west/up/down) |
| `look` | Examine current room |
| `take [item]` | Pick up an item |
| `drop [item]` | Drop an item from inventory |
| `use [item]` | Use an item |
| `examine [item]` | Inspect an item in detail |
| `read [document]` | Read a document |
| `combine [item1] [item2]` | Combine two items |
| `solve [puzzle]` | Attempt to solve a puzzle |
| `inventory` | View your inventory |
| `map` | View the facility map |
| `status` | View player statistics |
| `hint` | Get a contextual hint |
| `help` | Show all available commands |

### Tips

1. **Read everything** - Documents contain crucial clues and codes
2. **Explore thoroughly** - Some exits are hidden until conditions are met
3. **Combine items** - Some puzzles require combining items in specific rooms
4. **Use `hint`** - Stuck? The hint command provides context-aware guidance

---

## 🔧 Tech Stack

| Component | Technology |
|:---|:---|
| Game Engine | Rust |
| GUI Framework | Tauri v2 |
| Frontend | HTML5 / CSS3 / Vanilla JS |
| Styling | CSS Custom Properties, CSS Grid |
| Serialization | Serde + JSON |
| CI/CD | GitHub Actions |
| Target Platform | Windows (x86_64) |

### Architecture

```
RustTextAdventure/
├── src-tauri/              # Rust backend (Tauri)
│   ├── src/
│   │   ├── main.rs       # Tauri commands & app entry
│   │   └── game/
│   │       ├── engine.rs     # Core game engine
│   │       ├── types.rs      # Data types & structs
│   │       ├── world.rs      # Room definitions
│   │       ├── items.rs      # Item definitions
│   │       ├── puzzles.rs    # Puzzle definitions
│   │       ├── player.rs    # Player state
│   │       ├── commands.rs   # Command parser
│   │       ├── story.rs     # Narrative & achievements
│   │       └── save.rs      # Save/Load system
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── icons/
├── ui/                    # Frontend
│   ├── index.html
│   ├── css/style.css
│   └── js/app.js
└── .github/workflows/     # CI/CD
    └── release.yml
```

---

## 🏆 Achievements

| Achievement | How to Unlock |
|:---|:---|
| 🚶 First Steps | Enter the facility |
| 🗺️ Facility Explorer | Visit 5 rooms |
| 📚 Bookworm | Find all documents |
| 🗠️ Puzzle Master | Solve 3 puzzles |
| 🔐 Codebreaker | Crack the office code |
| 🧪 Amateur Chemist | Create the Stabilizer |
| ⚙️ Chief Engineer | Repair the generator |
| 💻 Elite Hacker | Decrypt the files |
| 💎 Facility Savior | Stabilize the Core |
| 🌍 Escape Artist | Reach the surface |
| ⭐ Hero | Complete the game |

---

## 🚀 Product Delivery and Validation

The global commercial roadmap is available in [`docs/GLOBAL_COMMERCIAL_ROADMAP_FA.md`](docs/GLOBAL_COMMERCIAL_ROADMAP_FA.md). The current, Tauri-specific delivery backlog is in [`docs/EXECUTION_BACKLOG_FA.md`](docs/EXECUTION_BACKLOG_FA.md), and the landing-page, interview, playtest, and attribution protocols are in [`docs/VALIDATION_TOOLKIT_FA.md`](docs/VALIDATION_TOOLKIT_FA.md).

---

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

<div align="center">

Built with ❤️ using Rust and Tauri

</div>