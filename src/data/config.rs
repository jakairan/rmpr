use serde::Deserialize;
use std::{fs, path::PathBuf};

/// Encapsulates themeing data.
#[derive(Deserialize, Clone)] // Clone is needed for lines 69-76 in tui and 140-141 in browser
#[serde(default)]
pub struct Colors {
    pub album: String,
    pub artist: String,
    pub border: String,
    pub fs_directory: String,
    pub fs_file: String,
    pub highlight_color: String,
    pub options: String,
    pub paused: String,
    pub playback_speed: String,
    pub seekbar_filled: String,
    pub seekbar_unfilled: String,
    pub status: String,
    pub tab_selected: String,
    pub tab_unselected: String,
    pub timestamp: String,
    pub title: String,
    pub track_num: String,
    pub volume: String,
    pub year: String,
}

impl Default for Colors {
    fn default() -> Self {
        Colors {
            album: String::from("#00FF00"),
            artist: String::from("#FF0000"),
            border: String::from("#FFFFFF"),
            fs_directory: String::from("#598EFF"),
            fs_file: String::from("#FFFFFF"),
            highlight_color: String::from("#FF0000"),
            options: String::from("#FF0000"),
            paused: String::from("#00FF00"),
            playback_speed: String::from("#598EFF"),
            seekbar_filled: String::from("#FFFFFF"),
            seekbar_unfilled: String::from("#000000"),
            status: String::from("#598EFF"),
            tab_selected: String::from("#FF0000"),
            tab_unselected: String::from("#FFFFFF"),
            timestamp: String::from("#598EFF"),
            title: String::from("#FFFF00"),
            track_num: String::from("#FF00FF"),
            volume: String::from("#598EFF"),
            year: String::from("#598EFF"),
        }
    }
}

/// Encapsulates directories data.
#[derive(Deserialize)]
#[serde(default)]
pub struct Directories {
    pub music_directory: PathBuf,
}

impl Default for Directories {
    fn default() -> Self {
        Directories {
            music_directory: dirs::home_dir()
                .map(|mut path| {
                    path.push("Music");
                    path
                })
                .unwrap(),
        }
    }
}

/// Encapsulates controlling data.
#[derive(Deserialize)]
#[serde(default)]
pub struct Controls {
    pub vol_delta: i16,
}

impl Default for Controls {
    fn default() -> Self {
        Controls { vol_delta: 2 }
    }
}

/// Encapsulates all config.toml parameters.
#[derive(Deserialize)]
pub struct ConfigData {
    pub colors: Colors,
    pub directories: Directories,
    pub controls: Controls,
}

impl Default for ConfigData {
    fn default() -> Self {
        ConfigData {
            colors: Colors::default(),
            directories: Directories::default(),
            controls: Controls::default(),
        }
    }
}

/// Loads the ConfigData from config.toml.
pub fn load_config() -> ConfigData {
    let config_path = dirs::config_dir()
        .map(|mut path| {
            path.push("rmpr/config.toml");
            path
        })
        .expect("Could not find home directory");

    let config_content = fs::read_to_string(&config_path)
        .unwrap_or_else(|_| panic!("Failed to read config file at {}", config_path.display()));

    toml::from_str(&config_content).unwrap_or_else(|_| panic!("Failed to parse TOML config"))
}
