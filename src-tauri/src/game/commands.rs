use super::engine::GameEngine;
use super::types::*;

// ============================================================
// Command Parser & Processor
// ============================================================

pub fn process_command(engine: &mut GameEngine, raw: &str) -> Vec<GameMessage> {
    let input = raw.trim().to_lowercase();
    if input.is_empty() {
        return vec![];
    }

    let parts: Vec<&str> = input.split_whitespace().collect();
    let verb = parts[0];
    let args = if parts.len() > 1 { parts[1..].join(" ") } else { String::new() };

    match verb {
        "help" | "h" | "?" => cmd_help(),
        "look" | "l" | "examine" | "inspect" => cmd_look(engine, &args),
        "go" | "move" | "walk" => cmd_go(engine, &args),
        "north" | "n" => cmd_go(engine, "north"),
        "south" | "s" => cmd_go(engine, "south"),
        "east" | "e" => cmd_go(engine, "east"),
        "west" | "w" => cmd_go(engine, "west"),
        "up" | "u" => cmd_go(engine, "up"),
        "down" | "d" => cmd_go(engine, "down"),
        "take" | "pick" | "grab" | "get" => cmd_take(engine, &args),
        "drop" | "discard" => cmd_drop(engine, &args),
        "use" => cmd_use(engine, &args),
        "combine" | "mix" => cmd_combine(engine, &args),
        "inventory" | "inv" | "i" => cmd_inventory(engine),
        "examine" | "x" => cmd_examine_item(engine, &args),
        "read" => cmd_read(engine, &args),
        "solve" | "code" => cmd_solve_puzzle(engine, &args),
        "map" | "m" => cmd_map(engine),
        "status" | "stats" => cmd_status(engine),
        "hint" => cmd_hint(engine),
        "restart" => cmd_restart(engine),
        _ => vec![GameMessage {
            text: format!("I don't understand '{}'. Type 'help' for available commands.", verb),
            msg_type: MessageType::Error, timestamp: String::new(),
        }],
    }
}

fn cmd_help() -> Vec<GameMessage> {
    vec![GameMessage {
        text: "Available Commands:\n\n\n  Navigation:  go [direction], north/n, south/s, east/e, west/w, up/u, down/d\n  
  Interaction: look [target], take [item], drop [item], use [item]\n  
  Items:      inventory/inv, examine [item], read [document]\n  
  Puzzles:    solve [puzzle], combine [item1] [item2]\n  
  Info:       help, map, status, hint\n  
  System:     restart".into(),
        msg_type: MessageType::System, timestamp: String::new(),
    }]
}

fn cmd_look(engine: &mut GameEngine, target: &str) -> Vec<GameMessage> {
    let mut msgs = Vec::new();
    if target.is_empty() || target == "room" || target == "around" {
        let room = engine.get_current_room();
        msgs.push(GameMessage { text: format!("\"{}\"", room.name), msg_type: MessageType::Description, timestamp: String::new() });
        msgs.push(GameMessage { text: room.description.clone(), msg_type: MessageType::Description, timestamp: String::new() });

        if !room.items.is_empty() {
            let item_names: Vec<String> = room.items.iter().filter_map(|id| {
                engine.all_items.iter().find(|i| i.id == *id).map(|i| format!("{} {}", i.icon, i.name))
            }).collect();
            msgs.push(GameMessage { text: format!("You can see: {}", item_names.join(", ")), msg_type: MessageType::Description, timestamp: String::new() });
        }

        let exit_names: Vec<String> = room.exits.iter().filter(|e| !e.hidden).map(|e| {
            let lock_info = if e.locked { " [locked]" } else { "" };
            format!("{} {}{}", e.direction.arrow(), e.direction.display(), lock_info)
        }).collect();
        msgs.push(GameMessage { text: format!("Exits: {}", exit_names.join("  ")), msg_type: MessageType::Description, timestamp: String::new() });

        engine.player.increment_moves();
    } else {
        if let Some(item) = room.items.iter().find(|id| *id == target || engine.all_items.iter().any(|i| i.id == *target && i.name.to_lowercase().contains(target)))
            .and_then(|id| engine.all_items.iter().find(|i| i.id == *id)) {
            msgs.push(GameMessage { text: format!("{} {}", item.icon, item.name), msg_type: MessageType::Description, timestamp: String::new() });
            msgs.push(GameMessage { text: item.detailed_description.clone(), msg_type: MessageType::Description, timestamp: String::new() });
        } else {
            msgs.push(GameMessage { text: format!("You don't see '{}' here.", target), msg_type: MessageType::Error, timestamp: String::new() });
        }
    }
    msgs
}

fn cmd_go(engine: &mut GameEngine, direction: &str) -> Vec<GameMessage> {
    let mut msgs = Vec::new();
    let dir = match Direction::from_str(direction) {
        Some(d) => d,
        None => return vec![GameMessage { text: format!("'{}' is not a valid direction.", direction), msg_type: MessageType::Error, timestamp: String::new() }],
    };

    let room_idx = engine.rooms.iter().position(|r| r.id == engine.current_room_id).unwrap();
    let exit = engine.rooms[room_idx].exits.iter().find(|e| e.direction == dir);

    match exit {
        None => vec![GameMessage { text: "You can't go that way.".into(), msg_type: MessageType::Error, timestamp: String::new() }],
        Some(exit) => {
            if exit.hidden && !engine.global_flags.contains(&"core_stabilized".to_string()) {
                return vec![GameMessage { text: "You can't go that way.".into(), msg_type: MessageType::Error, timestamp: String::new() }];
            }
            if exit.locked {
                if let Some(ref req_item) = exit.required_item {
                    if engine.player.has_item(req_item) {
                        let idx = engine.rooms[room_idx].exits.iter().position(|e| e.direction == dir).unwrap();
                        engine.rooms[room_idx].exits[idx].locked = false;
                        engine.player.remove_item(req_item);
                        msgs.push(GameMessage { text: format!("You use the required item to unlock the passage!"), msg_type: MessageType::Success, timestamp: String::new() });
                    } else {
                        return vec![GameMessage { text: "That passage is locked. You need a specific item to open it.".into(), msg_type: MessageType::Warning, timestamp: String::new() }];
                    }
                } else {
                    return vec![GameMessage { text: "That passage is locked.".into(), msg_type: MessageType::Warning, timestamp: String::new() }];
                }
            }

            let new_room_id = exit.target_room.clone();
            engine.current_room_id = new_room_id.clone();
            engine.player.increment_moves();

            let new_room_idx = engine.rooms.iter().position(|r| r.id == new_room_id).unwrap();
            let first_visit = !engine.rooms[new_room_idx].visited;
            engine.rooms[new_room_idx].visited = true;
            engine.rooms[new_room_idx].visit_count += 1;

            if first_visit {
                engine.player.rooms_explored += 1;
                engine.check_achievements(&mut msgs);
            }

            let room = &engine.rooms[new_room_idx];
            msgs.push(GameMessage { text: format!("--- {} ---", room.name), msg_type: MessageType::Description, timestamp: String::new() });
            msgs.push(GameMessage { text: room.description.clone(), msg_type: MessageType::Description, timestamp: String::new() });

            if first_visit {
                if let Some(visit_msg) = crate::game::story::get_room_first_visit_text(&room.id) {
                    msgs.push(visit_msg);
                }
            }

            if !room.items.is_empty() {
                let item_names: Vec<String> = room.items.iter().filter_map(|id| {
                    engine.all_items.iter().find(|i| i.id == *id).map(|i| format!("{} {}", i.icon, i.name))
                }).collect();
                msgs.push(GameMessage { text: format!("You can see: {}", item_names.join(", ")), msg_type: MessageType::Description, timestamp: String::new() });
            }

            let exit_names: Vec<String> = room.exits.iter().filter(|e| !e.hidden).map(|e| {
                let lock = if e.locked { " [locked]" } else { "" };
                format!("{}{}", e.direction.display(), lock)
            }).collect();
            msgs.push(GameMessage { text: format!("Exits: {}", exit_names.join(", ")), msg_type: MessageType::Description, timestamp: String::new() });

            if engine.config.auto_save && engine.player.moves % 10 == 0 {
                let _ = engine.auto_save();
            }

            if room.id == "exit_chamber" {
                engine.game_won = true;
                engine.score += 200;
                msgs.push(GameMessage { text: "\n=== CONGRATULATIONS! ===\nYou have escaped the Prometheus Research Facility! Your investigation has uncovered the truth about Project Prometheus. The world will know what happened here.".into(), msg_type: MessageType::Success, timestamp: String::new() });
                msgs.push(GameMessage { text: format!("Final Score: {} | Moves: {} | Rooms Explored: {} | Items: {} | Puzzles: {}",
                    engine.player.score, engine.player.moves, engine.player.rooms_explored,
                    engine.player.items_collected, engine.player.puzzles_solved), msg_type: MessageType::System, timestamp: String::new() });
            }

            msgs
        }
    }
}

fn cmd_take(engine: &mut GameEngine, target: &str) -> Vec<GameMessage> {
    let room_idx = engine.rooms.iter().position(|r| r.id == engine.current_room_id).unwrap();
    let target_lower = target.to_lowercase();

    let item_id = engine.rooms[room_idx].items.iter().find(|id| {
        engine.all_items.iter().any(|i| i.id == **id && (i.id == target_lower || i.name.to_lowercase().contains(&target_lower) || target_lower.contains(&i.id)))
    }).cloned();

    match item_id {
        None => vec![GameMessage { text: format!("You don't see '{}' here.", target), msg_type: MessageType::Error, timestamp: String::new() }],
        Some(id) => {
            let item = engine.all_items.iter().find(|i| i.id == id).unwrap().clone();
            if !item.takeable {
                return vec![GameMessage { text: "You can't take that.".into(), msg_type: MessageType::Error, timestamp: String::new() }];
            }
            engine.rooms[room_idx].items.retain(|i| i != &id);
            engine.player.add_item(item.clone());
            engine.player.increment_moves();
            engine.check_achievements(&mut Vec::new());
            vec![GameMessage { text: format!("{} You picked up the {}.", item.icon, item.name), msg_type: MessageType::Action, timestamp: String::new() }]
        }
    }
}

fn cmd_drop(engine: &mut GameEngine, target: &str) -> Vec<GameMessage> {
    let target_lower = target.to_lowercase();
    let item = engine.player.inventory.iter().find(|i| {
        i.id == target_lower || i.name.to_lowercase().contains(&target_lower)
    }).cloned();

    match item {
        None => vec![GameMessage { text: format!("You don't have '{}' in your inventory.", target), msg_type: MessageType::Error, timestamp: String::new() }],
        Some(item) => {
            engine.player.remove_item(&item.id);
            let room_idx = engine.rooms.iter().position(|r| r.id == engine.current_room_id).unwrap();
            engine.rooms[room_idx].items.push(item.id.clone());
            vec![GameMessage { text: format!("You dropped the {}.", item.name), msg_type: MessageType::Action, timestamp: String::new() }]
        }
    }
}

fn cmd_use(engine: &mut GameEngine, target: &str) -> Vec<GameMessage> {
    let target_lower = target.to_lowercase();
    let item = engine.player.inventory.iter().find(|i| {
        i.id == target_lower || i.name.to_lowercase().contains(&target_lower)
    });

    match item {
        None => vec![GameMessage { text: format!("You don't have '{}' to use.", target), msg_type: MessageType::Error, timestamp: String::new() }],
        Some(item) => {
            if !item.usable {
                return vec![GameMessage { text: format!("You can't use the {} directly.", item.name), msg_type: MessageType::Error, timestamp: String::new() }];
            }
            match item.id.as_str() {
                "signal_flare" if engine.current_room_id == "exit_chamber" => {
                    engine.player.remove_item(&item.id);
                    if let Some(puzzle) = engine.puzzles.iter_mut().find(|p| p.id == "signal_for_help") {
                        puzzle.solved = true;
                    }
                    engine.global_flags.push("rescue_called".into());
                    engine.score += 100;
                    vec![GameMessage { text: "You fire the signal flare! It rockets into the night sky, exploding in a brilliant red star visible for kilometers. Headlights respond in the distance - you're going home!".into(), msg_type: MessageType::Success, timestamp: String::new() }]
                }
                "binoculars" if engine.current_room_id == "observation_deck" => {
                    vec![GameMessage { text: "You raise the binoculars and scan the horizon. The communications tower stands on a distant ridge, its beacon still active. You can see a service road leading from the facility toward civilization. To the east, faint lights suggest a town.".into(), msg_type: MessageType::Description, timestamp: String::new() }]
                }
                "old_radio" => {
                    vec![GameMessage { text: "You crank the radio and tune to 147.3 MHz. Through the static, you hear a fragment: '...Prometheus Facility... anyone receiving... rescue team en route to coordinates...signal if you can hear...' The transmission fades back to static.".into(), msg_type: MessageType::Dialogue, timestamp: String::new() }]
                }
                _ => vec![GameMessage { text: format!("The {} can't be used here, or at this time.", item.name), msg_type: MessageType::Hint, timestamp: String::new() }],
            }
        }
    }
}

fn cmd_combine(engine: &mut GameEngine, args: &str) -> Vec<GameMessage> {
    let parts: Vec<&str> = args.split_whitespace().collect();
    if parts.len() < 2 {
        return vec![GameMessage { text: "Combine what? Usage: combine [item1] [item2]".into(), msg_type: MessageType::Error, timestamp: String::new() }];
    }

    let item1_name = parts[0].to_lowercase();
    let item2_name = parts[1..].join(" ").to_lowercase();

    if engine.current_room_id == "chemical_storage" {
        let has_compound = engine.player.has_item("strange_compound");
        let has_antidote = engine.player.has_item("antidote");
        let has_formula = engine.player.has_item("chemical_formula");

        if (item1_name.contains("strange") || item1_name.contains("compound") || item2_name.contains("strange") || item2_name.contains("compound"))
            && (item1_name.contains("antidote") || item2_name.contains("antidote")) {
            if has_compound && has_antidote && has_formula {
                engine.player.remove_item("strange_compound");
                engine.player.remove_item("antidote");
                if let Some(stab) = super::items::create_items().into_iter().find(|i| i.id == "prometheus_stabilizer") {
                    engine.player.add_item(stab);
                }
                engine.score += 75;
                if let Some(p) = engine.puzzles.iter_mut().find(|p| p.id == "create_stabilizer") {
                    p.solved = true;
                    engine.player.puzzles_solved += 1;
                }
                engine.global_flags.push("stabilizer_created".into());
                return vec![GameMessage { text: "The compounds react brilliantly! A shimmering iridescent liquid forms in the beaker - the Prometheus Stabilizer! You carefully transfer it to a vial.".into(), msg_type: MessageType::Success, timestamp: String::new() }];
            } else {
                let mut missing = Vec::new();
                if !has_compound { missing.push("Strange Compound"); }
                if !has_antidote { missing.push("Antidote"); }
                if !has_formula { missing.push("Chemical Formula"); }
                return vec![GameMessage { text: format!("You're missing: {}", missing.join(", ")), msg_type: MessageType::Hint, timestamp: String::new() }];
            }
        }
    }

    vec![GameMessage { text: "Those items can't be combined, or you're not in the right location.".into(), msg_type: MessageType::Hint, timestamp: String::new() }]
}

fn cmd_inventory(engine: &GameEngine) -> Vec<GameMessage> {
    if engine.player.inventory.is_empty() {
        return vec![GameMessage { text: "Your inventory is empty.".into(), msg_type: MessageType::System, timestamp: String::new() }];
    }
    let items: Vec<String> = engine.player.inventory.iter().map(|i| {
        format!("{} {} - {}", i.icon, i.name, i.description)
    }).collect();
    vec![GameMessage { text: format!("Inventory ({} items):\n{}", engine.player.inventory.len(), items.join("\n")), msg_type: MessageType::System, timestamp: String::new() }]
}

fn cmd_examine_item(engine: &GameEngine, target: &str) -> Vec<GameMessage> {
    let target_lower = target.to_lowercase();
    if let Some(item) = engine.player.inventory.iter().find(|i| i.id == target_lower || i.name.to_lowercase().contains(&target_lower)) {
        return vec![
            GameMessage { text: format!("{} {}", item.icon, item.name), msg_type: MessageType::Description, timestamp: String::new() },
            GameMessage { text: item.detailed_description.clone(), msg_type: MessageType::Description, timestamp: String::new() },
        ];
    }
    vec![GameMessage { text: format!("You don't have '{}' to examine.", target), msg_type: MessageType::Error, timestamp: String::new() }]
}

fn cmd_read(engine: &GameEngine, target: &str) -> Vec<GameMessage> {
    let target_lower = target.to_lowercase();
    let doc_ids = ["research_journal", "lab_notes", "chemical_formula", "decrypted_files", "final_note", "maintenance_manual", "photograph"];
    for id in &doc_ids {
        if engine.player.has_item(id) {
            if let Some(item) = engine.all_items.iter().find(|i| i.id == *id) {
                if item.id.contains(&target_lower) || item.name.to_lowercase().contains(&target_lower) || target.is_empty() {
                    return vec![GameMessage { text: format!("Reading '{}'...", item.name), msg_type: MessageType::Description, timestamp: String::new() },
                        GameMessage { text: item.detailed_description.clone(), msg_type: MessageType::Description, timestamp: String::new() }];
                }
            }
        }
    }
    vec![GameMessage { text: if target.is_empty() { "You don't have any readable documents.".into() } else { format!("You can't read '{}' or don't have it.", target) }, msg_type: MessageType::Error, timestamp: String::new() }]
}

fn cmd_solve_puzzle(engine: &mut GameEngine, args: &str) -> Vec<GameMessage> {
    let args_lower = args.to_lowercase();
    let mut msgs = Vec::new();

    if engine.current_room_id == "library" && (args_lower.contains("code") || args_lower.contains("7319")) {
        if args_lower.contains("7319") {
            let room_idx = engine.rooms.iter().position(|r| r.id == "library").unwrap();
            if let Some(exit) = engine.rooms[room_idx].exits.iter_mut().find(|e| e.target_room == "directors_office") {
                exit.locked = false;
            }
            engine.score += 50;
            engine.global_flags.push("office_code_known".into());
            if let Some(p) = engine.puzzles.iter_mut().find(|p| p.id == "office_code") { p.solved = true; engine.player.puzzles_solved += 1; }
            msgs.push(GameMessage { text: "The keypad beeps three times and the lock clicks open! The Director's Office door slides aside.".into(), msg_type: MessageType::Success, timestamp: String::new() });
        } else {
            msgs.push(GameMessage { text: "Enter the 4-digit code. Try 'solve 7319' if you found the code somewhere.".into(), msg_type: MessageType::Hint, timestamp: String::new() });
        }
        return msgs;
    }

    if engine.current_room_id == "experiment_chamber" && (args_lower.contains("stabilize") || args_lower.contains("core")) {
        if engine.player.has_item("prometheus_stabilizer") {
            engine.player.remove_item("prometheus_stabilizer");
            if let Some(core) = super::items::create_items().into_iter().find(|i| i.id == "prometheus_core") {
                engine.player.add_item(core);
            }
            engine.global_flags.push("core_stabilized".into());
            engine.score += 200;
            engine.player.puzzles_solved += 1;
            if let Some(p) = engine.puzzles.iter_mut().find(|p| p.id == "stabilize_core") { p.solved = true; }
            let room_idx = engine.rooms.iter().position(|r| r.id == "experiment_chamber").unwrap();
            if let Some(exit) = engine.rooms[room_idx].exits.iter_mut().find(|e| e.direction == Direction::North) {
                exit.hidden = false;
            }
            msgs.push(GameMessage { text: "You carefully apply the Stabilizer to the Core. The wild energy patterns calm, and the light stabilizes to a steady glow. A crystalline fragment breaks free. A concealed door hums open to the north...".into(), msg_type: MessageType::Success, timestamp: String::new() });
        } else {
            msgs.push(GameMessage { text: "The Core is dangerously unstable! You need the Prometheus Stabilizer to calm it.".into(), msg_type: MessageType::Warning, timestamp: String::new() });
        }
        return msgs;
    }

    if engine.current_room_id == "generator_room" && (args_lower.contains("repair") || args_lower.contains("install") || args_lower.contains("fix")) {
        let has_breaker = engine.player.has_item("circuit_breaker");
        let has_tools = engine.player.has_item("toolbox");
        if has_breaker && has_tools {
            engine.player.remove_item("circuit_breaker");
            engine.global_flags.push("generator_fixed".into());
            engine.score += 75;
            engine.player.puzzles_solved += 1;
            if let Some(p) = engine.puzzles.iter_mut().find(|p| p.id == "repair_generator") { p.solved = true; }
            msgs.push(GameMessage { text: "With practiced precision, you install the quantum breaker. The generator roars to full power! Lights throughout the facility brighten.".into(), msg_type: MessageType::Success, timestamp: String::new() });
        } else {
            let mut missing = Vec::new();
            if !has_breaker { missing.push("Circuit Breaker"); }
            if !has_tools { missing.push("Toolbox"); }
            msgs.push(GameMessage { text: format!("You need: {}", missing.join(", ")), msg_type: MessageType::Hint, timestamp: String::new() });
        }
        return msgs;
    }

    if engine.current_room_id == "network_hub" && (args_lower.contains("decrypt") || args_lower.contains("usb")) {
        if engine.player.has_item("usb_drive") {
            if let Some(files) = super::items::create_items().into_iter().find(|i| i.id == "decrypted_files") {
                engine.player.add_item(files);
            }
            engine.global_flags.push("files_decrypted".into());
            engine.score += 75;
            engine.player.puzzles_solved += 1;
            if let Some(p) = engine.puzzles.iter_mut().find(|p| p.id == "decrypt_usb") { p.solved = true; }
            msgs.push(GameMessage { text: "After minutes of processing, the decryption completes! The files reveal the truth about Project Prometheus.".into(), msg_type: MessageType::Success, timestamp: String::new() });
        } else {
            msgs.push(GameMessage { text: "You need the Encrypted USB Drive to use the decryption workstation.".into(), msg_type: MessageType::Hint, timestamp: String::new() });
        }
        return msgs;
    }

    let current_puzzles: Vec<&Puzzle> = engine.puzzles.iter().filter(|p| p.room_id == engine.current_room_id && !p.solved).collect();
    if current_puzzles.is_empty() {
        msgs.push(GameMessage { text: "There's no puzzle to solve here.".into(), msg_type: MessageType::System, timestamp: String::new() });
    } else {
        for p in &current_puzzles {
            msgs.push(GameMessage { text: format!("Available puzzle: {} - {}", p.name, p.description), msg_type: MessageType::Hint, timestamp: String::new() });
        }
    }
    msgs
}

fn cmd_map(engine: &GameEngine) -> Vec<GameMessage> {
    let mut lines = vec![];
    for room in &engine.rooms {
        let marker = if room.id == engine.current_room_id { " << YOU ARE HERE" } else if room.visited { " [visited]" } else { "" };
        lines.push(format!("{} {}", room.name, marker));
    }
    vec![GameMessage { text: format!("Facility Map:\n{}", lines.join("\n")), msg_type: MessageType::System, timestamp: String::new() }]
}

fn cmd_status(engine: &GameEngine) -> Vec<GameMessage> {
    let player = engine.player.get_state();
    vec![GameMessage {
        text: format!(
            "Player Status:\n  Health: {}/{}\n  Score: {}\n  Moves: {}\n  Rooms Explored: {}/{}\n  Items Collected: {}\n  Puzzles Solved: {}\n  Play Time: {}m",
            player.health, player.max_health, engine.player.score, player.moves,
            player.rooms_explored, engine.rooms.len(), player.items_collected,
            player.puzzles_solved, player.play_time_seconds / 60
        ),
        msg_type: MessageType::System, timestamp: String::new(),
    }]
}

fn cmd_hint(engine: &GameEngine) -> Vec<GameMessage> {
    let room_id = engine.current_room_id.as_str();
    let hint = match room_id {
        "entrance" => "Look around carefully. There might be something useful on the ground.",
        "main_hall" => "The north door requires a keycard. Check the entrance for one. Try exploring east and west first.",
        "library" => "Read the documents you find carefully. One of them contains a code.",
        "server_room" => "The network hub requires a biometric key. The director's office might have one.",
        "research_lab" => "Take the hazard suit - you'll need it for the experiment chamber.",
        "chemical_storage" => "If you have both compounds and the formula, try combining them here.",
        "experiment_chamber" => "The Core is unstable! Use 'solve stabilize' if you have the Stabilizer.",
        "generator_room" => "Use 'solve repair' if you have the circuit breaker and toolbox.",
        "basement_storage" => "The circuit breaker here is essential. Take it and the toolbox.",
        "network_hub" => "Use 'solve decrypt' with the USB drive to uncover the truth.",
        "observation_deck" => "Use the binoculars to scout the area.",
        "directors_office" => "Search everything. The director left important items behind.",
        _ => "Explore thoroughly and read all documents.",
    };
    vec![GameMessage { text: format!("Hint: {}", hint), msg_type: MessageType::Hint, timestamp: String::new() }]
}

fn cmd_restart(engine: &mut GameEngine) -> Vec<GameMessage> {
    engine.new_game();
    let mut msgs = crate::game::story::get_intro_text();
    let room = engine.get_current_room();
    msgs.push(GameMessage { text: format!("\n--- {} ---", room.name), msg_type: MessageType::Description, timestamp: String::new() });
    msgs.push(GameMessage { text: room.description.clone(), msg_type: MessageType::Description, timestamp: String::new() });
    msgs
}
