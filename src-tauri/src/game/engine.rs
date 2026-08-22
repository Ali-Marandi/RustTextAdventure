use super::commands;
use super::items;
use super::player::Player;
use super::puzzles;
use super::save;
use super::story;
use super::types::*;
use super::world;
use chrono::Local;

// ============================================================
// Main Game Engine
// ============================================================

pub struct GameEngine {
    pub rooms: Vec<Room>,
    pub all_items: Vec<Item>,
    pub puzzles: Vec<Puzzle>,
    pub achievements: Vec<Achievement>,
    pub player: Player,
    pub current_room_id: String,
    pub global_flags: Vec<String>,
    pub config: GameConfig,
    pub score: u32,
    pub game_over: bool,
    pub game_won: bool,
    pub message_history: Vec<GameMessage>,
}

impl GameEngine {
    pub fn new() -> Self {
        GameEngine {
            rooms: Vec::new(),
            all_items: Vec::new(),
            puzzles: Vec::new(),
            achievements: Vec::new(),
            player: Player::new(),
            current_room_id: String::new(),
            global_flags: Vec::new(),
            config: GameConfig::default(),
            score: 0,
            game_over: false,
            game_won: false,
            message_history: Vec::new(),
        }
    }

    pub fn new_game(&mut self) {
        self.rooms = world::create_world();
        self.all_items = items::create_items();
        self.puzzles = puzzles::create_puzzles();
        self.achievements = story::get_achievements();
        self.player = Player::new();
        self.current_room_id = "entrance".into();
        self.global_flags = Vec::new();
        self.score = 0;
        self.game_over = false;
        self.game_won = false;
        self.message_history = Vec::new();

        let room_idx = self.rooms.iter().position(|r| r.id == "entrance").unwrap();
        self.rooms[room_idx].visited = true;
        self.rooms[room_idx].visit_count = 1;
        self.player.rooms_explored = 1;
    }

    pub fn process_command(&mut self, command: &str) -> GameResponse {
        self.player.update_play_time();
        let mut new_messages = commands::process_command(self, command);
        for msg in &new_messages {
            self.message_history.push(msg.clone());
        }
        self.check_achievements(&mut new_messages);
        self.get_full_state_with_messages(new_messages)
    }

    pub fn get_full_state(&mut self) -> GameResponse {
        let mut msgs = story::get_intro_text();
        let room = self.get_current_room();
        msgs.push(GameMessage {
            text: format!("\n--- {} ---", room.name),
            msg_type: MessageType::Description,
            timestamp: String::new(),
        });
        msgs.push(GameMessage {
            text: room.description.clone(),
            msg_type: MessageType::Description,
            timestamp: String::new(),
        });

        if !room.items.is_empty() {
            let names: Vec<String> = room
                .items
                .iter()
                .filter_map(|id| {
                    self.all_items
                        .iter()
                        .find(|i| i.id == *id)
                        .map(|i| format!("{} {}", i.icon, i.name))
                })
                .collect();
            msgs.push(GameMessage {
                text: format!("You can see: {}", names.join(", ")),
                msg_type: MessageType::Description,
                timestamp: String::new(),
            });
        }

        let exits: Vec<String> = room
            .exits
            .iter()
            .filter(|e| !e.hidden)
            .map(|e| {
                let lock = if e.locked { " [locked]" } else { "" };
                format!("{} {}{}", e.direction.arrow(), e.direction.display(), lock)
            })
            .collect();
        msgs.push(GameMessage {
            text: format!("Exits: {}", exits.join("  ")),
            msg_type: MessageType::Description,
            timestamp: String::new(),
        });

        self.message_history = msgs.clone();
        self.get_full_state_with_messages(msgs)
    }

    /// Returns the current state without resetting the session or injecting a new-game intro.
    pub fn get_current_state(&self) -> GameResponse {
        self.get_full_state_with_messages(Vec::new())
    }

    pub fn get_config(&self) -> GameConfig {
        self.config.clone()
    }

    fn get_full_state_with_messages(&self, messages: Vec<GameMessage>) -> GameResponse {
        let room = self
            .rooms
            .iter()
            .find(|r| r.id == self.current_room_id)
            .unwrap();
        let now = Local::now().format("%H:%M:%S").to_string();
        let timestamped_msgs: Vec<GameMessage> = messages
            .into_iter()
            .map(|mut m| {
                if m.timestamp.is_empty() {
                    m.timestamp = now.clone();
                }
                m
            })
            .collect();

        let ground_items: Vec<InventoryItem> = room
            .items
            .iter()
            .filter_map(|id| {
                self.all_items
                    .iter()
                    .find(|i| i.id == *id)
                    .map(|i| InventoryItem {
                        id: i.id.clone(),
                        name: i.name.clone(),
                        description: i.description.clone(),
                        icon: i.icon.clone(),
                        category: format!("{:?}", i.category),
                        usable: i.usable,
                    })
            })
            .collect();

        let exit_states: Vec<ExitState> = room
            .exits
            .iter()
            .filter(|e| !e.hidden)
            .map(|e| ExitState {
                direction: e.direction.display().into(),
                arrow: e.direction.arrow().into(),
                description: e.description.clone(),
                locked: e.locked,
            })
            .collect();

        let map_positions = world::get_map_positions();
        let map_rooms: Vec<MapRoom> = self
            .rooms
            .iter()
            .map(|r| {
                let (x, y) = map_positions.get(&r.id).copied().unwrap_or((5.0, 5.0));
                MapRoom {
                    id: r.id.clone(),
                    name: r.name.clone(),
                    x,
                    y,
                    visited: r.visited,
                    is_current: r.id == self.current_room_id,
                }
            })
            .collect();

        let map_connections: Vec<MapConnection> = self
            .rooms
            .iter()
            .flat_map(|r| {
                r.exits
                    .iter()
                    .filter(|e| !e.hidden)
                    .map(|e| MapConnection {
                        from: r.id.clone(),
                        to: e.target_room.clone(),
                        direction: e.direction.display().into(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        let achievement_states: Vec<AchievementState> = self
            .achievements
            .iter()
            .map(|a| AchievementState {
                id: a.id.clone(),
                name: a.name.clone(),
                description: a.description.clone(),
                icon: a.icon.clone(),
                unlocked: a.unlocked,
                hidden: a.hidden,
            })
            .collect();

        GameResponse {
            messages: timestamped_msgs,
            current_room: RoomState {
                id: room.id.clone(),
                name: room.name.clone(),
                description: room.description.clone(),
                exits: exit_states,
                items_on_ground: ground_items,
                lighting: room.lighting.clone(),
                ambient: room.ambient_sound.clone().unwrap_or_default(),
            },
            inventory: self.player.get_inventory_items(),
            player: self.player.get_state(),
            map_data: MapData {
                rooms: map_rooms,
                current_room_id: self.current_room_id.clone(),
                connections: map_connections,
            },
            achievements: achievement_states,
            score: self.score,
            game_over: self.game_over,
            game_won: self.game_won,
        }
    }

    pub fn get_current_room(&self) -> &Room {
        self.rooms
            .iter()
            .find(|r| r.id == self.current_room_id)
            .unwrap()
    }

    pub fn check_achievements(&mut self, messages: &mut Vec<GameMessage>) {
        let now = Local::now().format("%Y-%m-%d %H:%M").to_string();

        let checks: Vec<(&str, bool, &str)> = vec![
            ("first_step", true, ""),
            ("explorer", self.player.rooms_explored >= 5, ""),
            (
                "cartographer",
                self.player.rooms_explored >= self.rooms.len() as u32,
                "",
            ),
            ("collector", self.player.items_collected >= 5, ""),
            (
                "bookworm",
                self.player
                    .inventory
                    .iter()
                    .filter(|i| matches!(i.category, ItemCategory::Document))
                    .count()
                    >= 5,
                "",
            ),
            ("puzzle_master", self.player.puzzles_solved >= 3, ""),
            (
                "codebreaker",
                self.global_flags.contains(&"office_code_known".to_string()),
                "",
            ),
            (
                "chemist",
                self.global_flags
                    .contains(&"stabilizer_created".to_string()),
                "",
            ),
            (
                "engineer",
                self.global_flags.contains(&"generator_fixed".to_string()),
                "",
            ),
            (
                "hacker",
                self.global_flags.contains(&"files_decrypted".to_string()),
                "",
            ),
            (
                "savior",
                self.global_flags.contains(&"core_stabilized".to_string()),
                "",
            ),
            ("escape_artist", self.current_room_id == "exit_chamber", ""),
            ("hero", self.game_won, ""),
        ];

        for (id, condition, _) in checks {
            if let Some(ach) = self.achievements.iter_mut().find(|a| a.id == id) {
                if condition && !ach.unlocked {
                    ach.unlocked = true;
                    ach.unlocked_at = Some(now.clone());
                    messages.push(GameMessage {
                        text: format!(
                            "Achievement Unlocked: {} {} - {}",
                            ach.icon, ach.name, ach.description
                        ),
                        msg_type: MessageType::Achievement,
                        timestamp: String::new(),
                    });
                }
            }
        }
    }

    pub fn save_to_slot(&mut self, slot: u32) -> CommandResult {
        let data = save::create_save_data(self, slot);
        match save::save_to_disk(&data) {
            Ok(()) => CommandResult::success(&format!("Game saved to slot {}.", slot + 1)),
            Err(e) => CommandResult::failure(&e),
        }
    }

    pub fn load_from_slot(&mut self, slot: u32) -> CommandResult {
        match save::load_from_disk(slot) {
            Ok(data) => {
                self.current_room_id = data.current_room_id;
                self.player.inventory = data.inventory;
                self.global_flags = data.global_flags;
                self.player.health = data.player_health;
                self.score = data.score;
                self.player.moves = data.moves;
                self.player.rooms_explored = data.rooms_explored;
                self.player.puzzles_solved = data.puzzles_solved;
                self.player.items_collected = data.items_collected;
                self.player.play_time_seconds = data.play_time_seconds;
                self.achievements = data.achievements;
                self.config = data.config;

                for (id, state) in &data.room_states {
                    if let Some(room) = self.rooms.iter_mut().find(|r| r.id == *id) {
                        room.visited = state.visited;
                        room.visit_count = state.visit_count;
                        room.items = state.items_remaining.clone();
                        room.flags = state.flags.clone();
                    }
                }
                for (id, solved) in &data.puzzle_states {
                    if let Some(puzzle) = self.puzzles.iter_mut().find(|p| p.id == *id) {
                        puzzle.solved = *solved;
                    }
                }
                CommandResult::success(&format!("Game loaded from slot {}.", slot + 1))
            }
            Err(e) => CommandResult::failure(&e),
        }
    }

    pub fn list_save_slots(&self) -> Vec<SaveSlotInfo> {
        save::list_save_slots_info()
            .into_iter()
            .map(|s| SaveSlotInfo {
                slot: s.slot,
                exists: s.exists,
                room_name: s.room_name,
                play_time: s.play_time,
                timestamp: s.timestamp,
            })
            .collect()
    }

    pub fn delete_save(&mut self, slot: u32) -> CommandResult {
        match save::delete_save_from_disk(slot) {
            Ok(()) => CommandResult::success(&format!("Save slot {} deleted.", slot + 1)),
            Err(e) => CommandResult::failure(&e),
        }
    }

    pub fn update_config(&mut self, config: GameConfig) {
        self.config = config;
    }

    pub fn get_achievements(&self) -> Vec<AchievementState> {
        self.achievements
            .iter()
            .map(|a| AchievementState {
                id: a.id.clone(),
                name: a.name.clone(),
                description: a.description.clone(),
                icon: a.icon.clone(),
                unlocked: a.unlocked,
                hidden: a.hidden,
            })
            .collect()
    }

    pub fn get_autocomplete_suggestions(&self, partial: &str) -> Vec<String> {
        let p = partial.to_lowercase();
        let mut suggestions = Vec::new();
        let base_commands = [
            "help",
            "look",
            "take",
            "drop",
            "use",
            "combine",
            "inventory",
            "examine",
            "read",
            "solve",
            "map",
            "status",
            "hint",
            "restart",
        ];
        for cmd in &base_commands {
            if cmd.starts_with(&p) {
                suggestions.push(cmd.to_string());
            }
        }
        let directions = ["north", "south", "east", "west", "up", "down"];
        for dir in &directions {
            if dir.starts_with(&p) {
                suggestions.push(dir.to_string());
            }
        }
        let room = self.get_current_room();
        for item_id in &room.items {
            if let Some(item) = self.all_items.iter().find(|i| i.id == *item_id) {
                if item.name.to_lowercase().contains(&p) || item.id.starts_with(&p) {
                    suggestions.push(format!("take {}", item.name.to_lowercase()));
                }
            }
        }
        for item in &self.player.inventory {
            if item.name.to_lowercase().contains(&p) || item.id.starts_with(&p) {
                suggestions.push(format!("use {}", item.name.to_lowercase()));
                suggestions.push(format!("examine {}", item.name.to_lowercase()));
            }
        }
        suggestions
    }

    pub fn auto_save(&self) -> Result<(), String> {
        let data = save::create_save_data(self, 0);
        save::save_to_disk(&data)
    }
}
