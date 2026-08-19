use super::types::*;

// ============================================================
// Story & Narrative System
// ============================================================

pub fn get_intro_text() -> Vec<GameMessage> {
    vec![
        GameMessage {
            text: "The year is 2037. You are Dr. Alex Chen, a renowned investigative journalist.".into(),
            msg_type: MessageType::Narrative, timestamp: String::new(),
        },
        GameMessage {
            text: "For months, you've been tracking rumors about the Prometheus Research Facility - a secret laboratory buried deep in the mountains, where revolutionary energy research was conducted before its sudden and mysterious abandonment.".into(),
            msg_type: MessageType::Narrative, timestamp: String::new(),
        },
        GameMessage {
            text: "Your investigation led you here, to a rusted entrance half-hidden by overgrowth. The facility's last known director, Dr. Helena Prometheus, vanished without a trace. Official records say the facility was closed due to 'budget constraints.' Your instincts tell you otherwise.".into(),
            msg_type: MessageType::Narrative, timestamp: String::new(),
        },
        GameMessage {
            text: "With a deep breath, you push open the heavy doors and step inside...".into(),
            msg_type: MessageType::Narrative, timestamp: String::new(),
        },
        GameMessage {
            text: "Welcome to THE RUST LABORATORY. Type 'help' for available commands, or 'look' to examine your surroundings.".into(),
            msg_type: MessageType::System, timestamp: String::new(),
        },
    ]
}

pub fn get_achievements() -> Vec<Achievement> {
    vec![
        Achievement {
            id: "first_step".into(),
            name: "First Steps".into(),
            description: "Enter the Prometheus Facility.".into(),
            icon: "\u{1f6b6}".into(),
            unlocked: false,
            unlocked_at: None,
            hidden: false,
        },
        Achievement {
            id: "explorer".into(),
            name: "Facility Explorer".into(),
            description: "Visit 5 different rooms.".into(),
            icon: "\u{1f5fa}".into(),
            unlocked: false,
            unlocked_at: None,
            hidden: false,
        },
        Achievement {
            id: "cartographer".into(),
            name: "Master Cartographer".into(),
            description: "Visit every room in the facility.".into(),
            icon: "\u{1f5fa}".into(),
            unlocked: false,
            unlocked_at: None,
            hidden: true,
        },
        Achievement {
            id: "collector".into(),
            name: "Hoarder".into(),
            description: "Collect 5 items.".into(),
            icon: "\u{1f392}".into(),
            unlocked: false,
            unlocked_at: None,
            hidden: false,
        },
        Achievement {
            id: "bookworm".into(),
            name: "Bookworm".into(),
            description: "Find and read all documents.".into(),
            icon: "\u{1f4da}".into(),
            unlocked: false,
            unlocked_at: None,
            hidden: true,
        },
        Achievement {
            id: "puzzle_master".into(),
            name: "Puzzle Master".into(),
            description: "Solve 3 puzzles.".into(),
            icon: "\u{1f9e0}".into(),
            unlocked: false,
            unlocked_at: None,
            hidden: false,
        },
        Achievement {
            id: "codebreaker".into(),
            name: "Codebreaker".into(),
            description: "Crack the Director's office code.".into(),
            icon: "\u{1f510}".into(),
            unlocked: false,
            unlocked_at: None,
            hidden: false,
        },
        Achievement {
            id: "chemist".into(),
            name: "Amateur Chemist".into(),
            description: "Create the Prometheus Stabilizer.".into(),
            icon: "\u{1f9ea}".into(),
            unlocked: false,
            unlocked_at: None,
            hidden: false,
        },
        Achievement {
            id: "engineer".into(),
            name: "Chief Engineer".into(),
            description: "Repair the facility's generator.".into(),
            icon: "\u{2699}".into(),
            unlocked: false,
            unlocked_at: None,
            hidden: false,
        },
        Achievement {
            id: "hacker".into(),
            name: "Elite Hacker".into(),
            description: "Decrypt the classified files.".into(),
            icon: "\u{1f4bb}".into(),
            unlocked: false,
            unlocked_at: None,
            hidden: false,
        },
        Achievement {
            id: "savior".into(),
            name: "Facility Savior".into(),
            description: "Stabilize the Prometheus Core.".into(),
            icon: "\u{1f48e}".into(),
            unlocked: false,
            unlocked_at: None,
            hidden: true,
        },
        Achievement {
            id: "escape_artist".into(),
            name: "Escape Artist".into(),
            description: "Reach the surface exit.".into(),
            icon: "\u{1f30d}".into(),
            unlocked: false,
            unlocked_at: None,
            hidden: true,
        },
        Achievement {
            id: "hero".into(),
            name: "Hero".into(),
            description: "Complete the game.".into(),
            icon: "\u{2b50}".into(),
            unlocked: false,
            unlocked_at: None,
            hidden: true,
        },
    ]
}

pub fn get_room_first_visit_text(room_id: &str) -> Option<GameMessage> {
    match room_id {
        "main_hall" => Some(GameMessage {
            text: "The blast doors seal shut behind you with a definitive CLANG. The holographic display in the center flickers to life, displaying: 'WELCOME BACK, DR. PROMETHEUS.' Something is very wrong here.".into(),
            msg_type: MessageType::Warning, timestamp: String::new(),
        }),
        "library" => Some(GameMessage {
            text: "The library's systems activate as you enter. A soft chime indicates that the archive system is still partially operational.".into(),
            msg_type: MessageType::Description, timestamp: String::new(),
        }),
        "server_room" => Some(GameMessage {
            text: "The temperature drops noticeably as you enter. The servers are working overtime, their cooling systems barely keeping up.".into(),
            msg_type: MessageType::Description, timestamp: String::new(),
        }),
        "experiment_chamber" => Some(GameMessage {
            text: "A wave of energy washes over you as the chamber door opens. The Prometheus Core pulses with unstable power, casting shifting shadows across the walls. You feel the hair on your arms stand on end.".into(),
            msg_type: MessageType::Warning, timestamp: String::new(),
        }),
        "exit_chamber" => Some(GameMessage {
            text: "Fresh air! You can see stars through the opening above. The facility's communication tower stands on a nearby ridge, its beacon still blinking faithfully after all these years.".into(),
            msg_type: MessageType::Success, timestamp: String::new(),
        }),
        _ => None,
    }
}
