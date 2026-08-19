use super::engine::GameEngine;
use super::types::*;
use chrono::Local;

// ============================================================
// Save/Load System
// ============================================================

pub fn create_save_data(engine: &GameEngine, slot: u32) -> SaveData {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let current_room = engine.rooms.iter().find(|r| r.id == engine.current_room_id);
    let room_name = current_room.map(|r| r.name.clone()).unwrap_or_default();

    let room_states: std::collections::HashMap<String, SavedRoomState> = engine
        .rooms
        .iter()
        .map(|r| {
            (
                r.id.clone(),
                SavedRoomState {
                    visited: r.visited,
                    visit_count: r.visit_count,
                    items_remaining: r.items.clone(),
                    flags: r.flags.clone(),
                    exits_unlocked: r
                        .exits
                        .iter()
                        .filter(|e| !e.locked)
                        .map(|_| "unlocked".to_string())
                        .collect(),
                },
            )
        })
        .collect();

    let puzzle_states: std::collections::HashMap<String, bool> = engine
        .puzzles
        .iter()
        .map(|p| (p.id.clone(), p.solved))
        .collect();

    SaveData {
        version: 2,
        slot,
        current_room_id: engine.current_room_id.clone(),
        inventory: engine.player.inventory.clone(),
        room_states,
        puzzle_states,
        global_flags: engine.global_flags.clone(),
        player_health: engine.player.health,
        score: engine.player.score,
        moves: engine.player.moves,
        rooms_explored: engine.player.rooms_explored,
        puzzles_solved: engine.player.puzzles_solved,
        items_collected: engine.player.items_collected,
        play_time_seconds: engine.player.play_time_seconds,
        achievements: engine.achievements.clone(),
        config: engine.config.clone(),
        saved_at: now.clone(),
        room_name,
    }
}

pub fn save_to_disk(data: &SaveData) -> Result<(), String> {
    let dir = dirs::data_local_dir()
        .ok_or("Cannot access local data directory")?
        .join("rust-text-adventure");
    std::fs::create_dir_all(&dir).map_err(|e| format!("Failed to create save directory: {}", e))?;

    let file_path = dir.join(format!("save_slot_{}.json", data.slot));
    let json = serde_json::to_string_pretty(data)
        .map_err(|e| format!("Failed to serialize save: {}", e))?;

    std::fs::write(&file_path, json).map_err(|e| format!("Failed to write save: {}", e))?;

    Ok(())
}

pub fn load_from_disk(slot: u32) -> Result<SaveData, String> {
    let dir = dirs::data_local_dir()
        .ok_or("Cannot access local data directory")?
        .join("rust-text-adventure");

    let file_path = dir.join(format!("save_slot_{}.json", slot));
    let json = std::fs::read_to_string(&file_path)
        .map_err(|e| format!("No save found in slot {}: {}", slot, e))?;

    serde_json::from_str(&json).map_err(|e| format!("Failed to parse save: {}", e))
}

pub fn delete_save_from_disk(slot: u32) -> Result<(), String> {
    let dir = dirs::data_local_dir()
        .ok_or("Cannot access local data directory")?
        .join("rust-text-adventure");

    let file_path = dir.join(format!("save_slot_{}.json", slot));
    if file_path.exists() {
        std::fs::remove_file(&file_path).map_err(|e| format!("Failed to delete save: {}", e))?;
    }
    Ok(())
}

pub fn list_save_slots_info() -> Vec<SaveSlotInfo> {
    let mut slots = Vec::new();
    for slot in 0..3 {
        match load_from_disk(slot) {
            Ok(data) => slots.push(SaveSlotInfo {
                slot,
                exists: true,
                room_name: data.room_name,
                play_time: format_play_time(data.play_time_seconds),
                timestamp: data.saved_at,
            }),
            Err(_) => slots.push(SaveSlotInfo {
                slot,
                exists: false,
                room_name: "Empty".into(),
                play_time: "--:--".into(),
                timestamp: String::new(),
            }),
        }
    }
    slots
}

fn format_play_time(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}
