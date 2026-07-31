# Rust Laboratory Adventure

A professional, creative, and robust text-based adventure game built with Rust. This project demonstrates the power of Rust's ownership model, pattern matching, and efficient data handling in a creative context.

## Features

- **Dynamic Room System:** Easily extensible room and exit configuration.
- **Command-Line Interface:** Simple and intuitive text-based interaction.
- **State Management:** Robust handling of game state, including player location and inventory.
- **Rust Best Practices:** Clean, idiomatic Rust code focusing on safety and performance.

## Installation

Ensure you have Rust and Cargo installed.

```bash
git clone https://github.com/Ali-Marandi/RustTextAdventure.git
cd RustTextAdventure
cargo build --release
```

## How to Play

Run the game using:

```bash
cargo run
```

### Commands

- `go [direction]`: Move to a connected room (e.g., `go north`).
- `inventory`: View your current items.
- `help`: Display available commands.
- `quit` or `exit`: End the game.

## Creative Vision

This project serves as a foundation for a larger narrative-driven exploration game. The use of Rust ensures that the game is not only fast but also free from common memory-related bugs, allowing for a seamless player experience.

## License

MIT License
