// The Rust Laboratory - Commercial Text Adventure Engine
// A full-featured text adventure game with Tauri GUI

mod game;

use game::{
    engine::GameEngine,
    types::{CommandResult, GameConfig, GameResponse, SaveSlotInfo},
};
use std::sync::Mutex;
use tauri::State;

struct GameState(Mutex<GameEngine>);

// ============================================================
// Tauri Commands - Frontend calls these via invoke()
// ============================================================

#[tauri::command]
fn initialize_game(state: &State<GameState>) -> GameResponse {
    let mut engine = state.0.lock().unwrap();
    engine.new_game();
    engine.get_full_state()
}

#[tauri::command]
fn send_command(state: &State<GameState>, command: String) -> GameResponse {
    let mut engine = state.0.lock().unwrap();
    engine.process_command(&command)
}

#[tauri::command]
fn save_game(state: &State<GameState>, slot: u32) -> CommandResult {
    let mut engine = state.0.lock().unwrap();
    engine.save_to_slot(slot)
}

#[tauri::command]
fn load_game(state: &State<GameState>, slot: u32) -> CommandResult {
    let mut engine = state.0.lock().unwrap();
    engine.load_from_slot(slot)
}

#[tauri::command]
fn get_save_slots(state: &State<GameState>) -> Vec<SaveSlotInfo> {
    let engine = state.0.lock().unwrap();
    engine.list_save_slots()
}

#[tauri::command]
fn delete_save(state: &State<GameState>, slot: u32) -> CommandResult {
    let mut engine = state.0.lock().unwrap();
    engine.delete_save(slot)
}

#[tauri::command]
fn update_config(state: &State<GameState>, config: GameConfig) -> CommandResult {
    let mut engine = state.0.lock().unwrap();
    engine.update_config(config);
    CommandResult::success("Settings updated successfully.".to_string())
}

#[tauri::command]
fn get_achievements(state: &State<GameState>) -> serde_json::Value {
    let engine = state.0.lock().unwrap();
    serde_json::to_value(engine.get_achievements()).unwrap_or_default()
}

#[tauri::command]
fn get_autocomplete(state: &State<GameState>, partial: String) -> Vec<String> {
    let engine = state.0.lock().unwrap();
    engine.get_autocomplete_suggestions(&partial)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(GameState(Mutex::new(GameEngine::new())))
        .invoke_handler(tauri::generate_handler![
            initialize_game,
            send_command,
            save_game,
            load_game,
            get_save_slots,
            delete_save,
            update_config,
            get_achievements,
            get_autocomplete,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
