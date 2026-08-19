use crate::game::GameState;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    pub text_scale: f32,
    pub high_contrast: bool,
    pub reduce_motion: bool,
    pub show_hints: bool,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            text_scale: 1.0,
            high_contrast: false,
            reduce_motion: false,
            show_hints: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveBundle {
    pub app_version: String,
    pub game: GameState,
    pub settings: UserSettings,
}

pub struct LocalStore;

impl LocalStore {
    fn data_dir() -> Result<PathBuf, String> {
        ProjectDirs::from("com", "AliMarandi", "RustLaboratorySignalZero")
            .map(|dirs| dirs.data_local_dir().to_path_buf())
            .ok_or_else(|| "Unable to find a local application data directory.".to_string())
    }

    fn save_path() -> Result<PathBuf, String> {
        Ok(Self::data_dir()?.join("savegame.json"))
    }

    fn settings_path() -> Result<PathBuf, String> {
        Ok(Self::data_dir()?.join("settings.json"))
    }

    fn write_json<T: Serialize>(path: PathBuf, value: &T) -> Result<(), String> {
        let parent = path
            .parent()
            .ok_or_else(|| "The local data path is invalid.".to_string())?;
        fs::create_dir_all(parent)
            .map_err(|error| format!("Unable to create local data directory: {error}"))?;
        let serialized = serde_json::to_string_pretty(value)
            .map_err(|error| format!("Unable to encode local data: {error}"))?;
        let temporary = path.with_extension("tmp");
        fs::write(&temporary, serialized)
            .map_err(|error| format!("Unable to write local data: {error}"))?;
        fs::rename(&temporary, &path)
            .map_err(|error| format!("Unable to finalize local data: {error}"))
    }

    fn read_json<T: for<'de> Deserialize<'de>>(path: PathBuf) -> Result<Option<T>, String> {
        if !path.exists() {
            return Ok(None);
        }
        let raw = fs::read_to_string(&path)
            .map_err(|error| format!("Unable to read local data: {error}"))?;
        let decoded = serde_json::from_str(&raw)
            .map_err(|error| format!("Unable to decode local data: {error}"))?;
        Ok(Some(decoded))
    }

    pub fn save_game(game: &GameState, settings: &UserSettings) -> Result<(), String> {
        Self::write_json(
            Self::save_path()?,
            &SaveBundle {
                app_version: env!("CARGO_PKG_VERSION").to_owned(),
                game: game.clone(),
                settings: settings.clone(),
            },
        )
    }

    pub fn load_game() -> Result<Option<SaveBundle>, String> {
        Self::read_json(Self::save_path()?)
    }

    pub fn save_settings(settings: &UserSettings) -> Result<(), String> {
        Self::write_json(Self::settings_path()?, settings)
    }

    pub fn load_settings() -> Result<UserSettings, String> {
        Ok(Self::read_json(Self::settings_path()?)?.unwrap_or_default())
    }
}
