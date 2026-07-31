use std::io::{self, Write};
use std::collections::HashMap;

struct Room {
    description: String,
    exits: HashMap<String, String>,
}

struct GameState {
    current_room: String,
    inventory: Vec<String>,
    rooms: HashMap<String, Room>,
    is_running: bool,
}

impl GameState {
    fn new() -> Self {
        let mut rooms = HashMap::new();
        
        rooms.insert(
            "Entrance".to_string(),
            Room {
                description: "You are at the entrance of a mysterious laboratory. The air is cold and smells of ozone.".to_string(),
                exits: [("north".to_string(), "MainHall".to_string())].iter().cloned().collect(),
            },
        );
        
        rooms.insert(
            "MainHall".to_string(),
            Room {
                description: "A vast hall with flickering lights. To the east is a library, and to the west is a server room.".to_string(),
                exits: [
                    ("south".to_string(), "Entrance".to_string()),
                    ("east".to_string(), "Library".to_string()),
                    ("west".to_string(), "ServerRoom".to_string()),
                ].iter().cloned().collect(),
            },
        );

        rooms.insert(
            "Library".to_string(),
            Room {
                description: "Shelves full of ancient books and digital scrolls. There's a strange glow coming from a terminal.".to_string(),
                exits: [("west".to_string(), "MainHall".to_string())].iter().cloned().collect(),
            },
        );

        rooms.insert(
            "ServerRoom".to_string(),
            Room {
                description: "Humming servers line the walls. A console is active, waiting for input.".to_string(),
                exits: [("east".to_string(), "MainHall".to_string())].iter().cloned().collect(),
            },
        );

        GameState {
            current_room: "Entrance".to_string(),
            inventory: Vec::new(),
            rooms,
            is_running: true,
        }
    }

    fn run(&mut self) {
        println!("--- Welcome to the Rust Laboratory Adventure ---");
        while self.is_running {
            self.display_current_room();
            let input = self.get_input();
            self.process_command(&input);
        }
        println!("Thank you for playing!");
    }

    fn display_current_room(&self) {
        let room = &self.rooms[&self.current_room];
        println!("\n[{}]", self.current_room);
        println!("{}", room.description);
        print!("Exits: ");
        for exit in room.exits.keys() {
            print!("{} ", exit);
        }
        println!();
    }

    fn get_input(&self) -> String {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_lowercase()
    }

    fn process_command(&mut self, command: &str) {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() { return; }

        match parts[0] {
            "go" => {
                if parts.len() > 1 {
                    self.move_to(parts[1]);
                } else {
                    println!("Go where?");
                }
            }
            "quit" | "exit" => self.is_running = false,
            "help" => println!("Commands: go [direction], inventory, quit, help"),
            "inventory" => {
                if self.inventory.is_empty() {
                    println!("Your inventory is empty.");
                } else {
                    println!("Inventory: {:?}", self.inventory);
                }
            }
            _ => println!("I don't understand that command."),
        }
    }

    fn move_to(&mut self, direction: &str) {
        let current = &self.rooms[&self.current_room];
        if let Some(next_room) = current.exits.get(direction) {
            self.current_room = next_room.clone();
        } else {
            println!("You can't go that way.");
        }
    }
}

fn main() {
    let mut game = GameState::new();
    game.run();
}
