use serde::{Deserialize, Serialize};

// ============================================================
// Core Data Types for the Game Engine
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
    Northeast,
    Northwest,
    Southeast,
    Southwest,
}

impl Direction {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "north" | "n" => Some(Direction::North),
            "south" | "s" => Some(Direction::South),
            "east" | "e" => Some(Direction::East),
            "west" | "w" => Some(Direction::West),
            "up" | "u" => Some(Direction::Up),
            "down" | "d" => Some(Direction::Down),
            "northeast" | "ne" => Some(Direction::Northeast),
            "northwest" | "nw" => Some(Direction::Northwest),
            "southeast" | "se" => Some(Direction::Southeast),
            "southwest" | "sw" => Some(Direction::Southwest),
            _ => None,
        }
    }

    pub fn display(&self) -> &str {
        match self {
            Direction::North => "North",
            Direction::South => "South",
            Direction::East => "East",
            Direction::West => "West",
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Northeast => "Northeast",
            Direction::Northwest => "Northwest",
            Direction::Southeast => "Southeast",
            Direction::Southwest => "Southwest",
        }
    }

    pub fn arrow(&self) -> &str {
        match self {
            Direction::North => "\u{2191}",
            Direction::South => "\u{2193}",
            Direction::East => "\u{2192}",
            Direction::West => "\u{2190}",
            Direction::Up => "\u{2191}",
            Direction::Down => "\u{2193}",
            Direction::Northeast => "\u{2197}",
            Direction::Northwest => "\u{2196}",
            Direction::Southeast => "\u{2198}",
            Direction::Southwest => "\u{2199}",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub description: String,
    pub detailed_description: String,
    pub exits: Vec<Exit>,
    pub items: Vec<String>,      // Item IDs on the ground
    pub flags: Vec<String>,      // Room state flags
    pub visited: bool,
    pub visit_count: u32,
    pub ambient_sound: Option<String>,
    pub lighting: LightingLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exit {
    pub direction: Direction,
    pub target_room: String,
    pub description: Option<String>,
    pub locked: bool,
    pub required_item: Option<String>,
    pub hidden: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LightingLevel {
    Bright,
    Dim,
    Dark,
    PitchBlack,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
    pub detailed_description: String,
    pub category: ItemCategory,
    pub usable: bool,
    pub combinable: bool,
    pub combine_result: Option<String>,
    pub combine_with: Option<String>,
    pub use_target: Option<String>,  // Room or item ID where it can be used
    pub weight: f32,
    pub icon: String,               // Emoji or unicode symbol
    pub takeable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ItemCategory {
    Key,
    Tool,
    Document,
    Consumable,
    PuzzlePiece,
    Treasure,
    Quest,
    Container,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Puzzle {
    pub id: String,
    pub name: String,
    pub description: String,
    pub room_id: String,
    pub puzzle_type: PuzzleType,
    pub required_items: Vec<String>,
    pub solution: String,
    pub solved: bool,
    pub reward_item: Option<String>,
    pub reward_flag: Option<String>,
    pub hint: String,
    pub failure_message: String,
    pub success_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PuzzleType {
    ItemCombination,
    CodeEntry,
    SequencePattern,
    LogicRiddle,
    PhysicalInteraction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NPC {
    pub id: String,
    pub name: String,
    pub description: String,
    pub room_id: String,
    pub dialogue: Vec<DialogueEntry>,
    pub friendly: bool,
    pub gives_item: Option<String>,
    pub gives_condition: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueEntry {
    pub id: String,
    pub speaker: String,
    pub text: String,
    pub condition: Option<String>,
    pub next_id: Option<String>,
    pub is_end: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub unlocked: bool,
    pub unlocked_at: Option<String>,
    pub hidden: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameConfig {
    pub text_speed: u32,          // ms per character
    pub sound_volume: f32,        // 0.0 - 1.0
    pub music_volume: f32,        // 0.0 - 1.0
    pub font_size: u32,           // px
    pub theme: String,            // "dark", "cyberpunk", "retro"
    pub show_minimap: bool,
    pub auto_save: bool,
    pub confirm_actions: bool,
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            text_speed: 30,
            sound_volume: 0.7,
            music_volume: 0.5,
            font_size: 16,
            theme: "cyberpunk".to_string(),
            show_minimap: true,
            auto_save: true,
            confirm_actions: true,
        }
    }
}

// Save slot info for the load/save UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveSlotInfo {
    pub slot: u32,
    pub exists: bool,
    pub room_name: String,
    pub play_time: String,
    pub timestamp: String,
}

// ============================================================
// Response types sent to the frontend
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameResponse {
    pub messages: Vec<GameMessage>,
    pub current_room: RoomState,
    pub inventory: Vec<InventoryItem>,
    pub player: PlayerState,
    pub map_data: MapData,
    pub achievements: Vec<AchievementState>,
    pub score: u32,
    pub game_over: bool,
    pub game_won: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMessage {
    pub text: String,
    pub msg_type: MessageType,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    Narrative,
    Description,
    Action,
    Error,
    System,
    Dialogue,
    Achievement,
    Hint,
    Warning,
    Success,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomState {
    pub id: String,
    pub name: String,
    pub description: String,
    pub exits: Vec<ExitState>,
    pub items_on_ground: Vec<InventoryItem>,
    pub lighting: LightingLevel,
    pub ambient: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExitState {
    pub direction: String,
    pub arrow: String,
    pub description: Option<String>,
    pub locked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryItem {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub category: String,
    pub usable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerState {
    pub health: i32,
    pub max_health: i32,
    pub moves: u32,
    pub rooms_explored: u32,
    pub puzzles_solved: u32,
    pub items_collected: u32,
    pub play_time_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapData {
    pub rooms: Vec<MapRoom>,
    pub current_room_id: String,
    pub connections: Vec<MapConnection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapRoom {
    pub id: String,
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub visited: bool,
    pub is_current: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapConnection {
    pub from: String,
    pub to: String,
    pub direction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementState {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub unlocked: bool,
    pub hidden: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub success: bool,
    pub message: String,
}

impl CommandResult {
    pub fn success(msg: &str) -> Self {
        CommandResult {
            success: true,
            message: msg.to_string(),
        }
    }

    pub fn failure(msg: &str) -> Self {
        CommandResult {
            success: false,
            message: msg.to_string(),
        }
    }
}

// Full save state for serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub version: u32,
    pub slot: u32,
    pub current_room_id: String,
    pub inventory: Vec<Item>,
    pub room_states: std::collections::HashMap<String, SavedRoomState>,
    pub puzzle_states: std::collections::HashMap<String, bool>,
    pub global_flags: Vec<String>,
    pub player_health: i32,
    pub score: u32,
    pub moves: u32,
    pub rooms_explored: u32,
    pub puzzles_solved: u32,
    pub items_collected: u32,
    pub play_time_seconds: u64,
    pub achievements: Vec<Achievement>,
    pub config: GameConfig,
    pub saved_at: String,
    pub room_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedRoomState {
    pub visited: bool,
    pub visit_count: u32,
    pub items_remaining: Vec<String>,
    pub flags: Vec<String>,
    pub exits_unlocked: Vec<String>,
}
