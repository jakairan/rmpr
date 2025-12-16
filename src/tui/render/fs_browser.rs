use crate::{
    data::{
        config::{ConfigData, load_config},
        metadata::file_metadata::FileMetadata,
    },
    tui::app::PLAYABLE,
};
use ratatui::{
    style::{Color, Style},
    widgets::{ListItem, ListState},
};
use std::{collections::HashMap, error::Error, fs::read_dir, path::PathBuf, str::FromStr};

/// Encapsulates file system browsing state and behavior.
pub struct FileBrowser {
    pub config: ConfigData,
    pub current_dir: PathBuf,
    pub entries: Vec<PathBuf>,
    pub list_state: ListState,
    pub sel_map: HashMap<PathBuf, usize>,
    pub selected: usize,
}

impl FileBrowser {
    pub fn new(initial_dir: PathBuf) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        let mut sel_map = HashMap::new();
        sel_map.insert(initial_dir.clone(), 0);

        Self {
            config: load_config(),
            current_dir: initial_dir,
            entries: Vec::new(),
            list_state,
            selected: 0,
            sel_map,
        }
    }

    /// Returns true if the file begins with '.'
    fn is_hidden(path: &PathBuf) -> bool {
        path.file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.starts_with('.'))
            .unwrap_or(false)
    }

    /// Returns true if the file's extention is in PLAYABLE
    fn is_playable_file(path: &PathBuf, playable_exts: &[&str]) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| playable_exts.contains(&ext.to_ascii_lowercase().as_str()))
            .unwrap_or(false)
    }

    /// Refreshes the list of entries from the current directory.
    pub fn update_entries(&mut self) -> Result<(), Box<dyn Error>> {
        let (mut directories, audio_files): (Vec<PathBuf>, Vec<PathBuf>) =
            read_dir(&self.current_dir)?
                .filter_map(|entry| {
                    entry
                        .ok()
                        .filter(|entry| !Self::is_hidden(&entry.path()))
                        .map(|entry| entry.path())
                })
                .partition(|path| path.is_dir());

        directories.sort_unstable();

        let mut metadata_list: Vec<(u16, String, PathBuf)> = audio_files
            .into_iter()
            .filter(|file| Self::is_playable_file(file, &PLAYABLE))
            .map(|path| {
                let file_data = FileMetadata::get_file_data(&path);
                (
                    file_data.track_number.unwrap_or(0),
                    file_data.title.unwrap_or(file_data.raw_file),
                    path,
                )
            })
            .collect();

        metadata_list.sort_unstable_by_key(|entry| entry.0);

        self.entries = directories
            .into_iter()
            .chain(metadata_list.into_iter().map(|(_, _, path)| path))
            .collect();

        self.list_state
            .select((!self.entries.is_empty()).then_some(self.selected));

        Ok(())
    }

    /// Moves the cursor up one element or goes to the bottom if at the top.
    pub fn navigate_up(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        if let 0 = self.selected {
            self.selected = self.entries.len() - 1
        } else {
            self.selected -= 1
        }
        self.sel_map.insert(self.current_dir.clone(), self.selected);
    }

    /// Moves the cursor down one element or goes to the top if at the bottom.
    pub fn navigate_down(&mut self) {
        if self.entries.is_empty() {
            return;
        }
        if self.selected < self.entries.len() - 1 {
            self.selected += 1
        } else {
            self.selected = 0
        }
        self.sel_map.insert(self.current_dir.clone(), self.selected);
    }

    /// Navigates into the selected directory, either setting the cursor to the saved position or 0.
    pub fn navigate_into(&mut self) {
        if let Some(path) = self.entries.get(self.selected) {
            if path.is_dir() {
                self.current_dir = path.clone();
                self.selected = *self.sel_map.get(&self.current_dir).unwrap_or(&0);
            }
        }
    }

    /// Navigates into the previous directory, either setting the cursor to the saved position or 0.
    pub fn navigate_back(&mut self) {
        if self.current_dir == self.config.directories.music_directory {
            return;
        }
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
            self.selected = *self.sel_map.get(&self.current_dir).unwrap_or(&0);
        }
    }

    /// Moves the cursor to the top of the list.
    pub fn goto_top(&mut self) {
        self.selected = 0
    }

    /// Moves the cursor to the bottom of the list.
    pub fn goto_bottom(&mut self) {
        self.selected = self.entries.len();
    }

    /// Navigates to the user's set music directory and sets selected to the top (1).
    pub fn goto_music_dir(&mut self) {
        if self.current_dir == self.config.directories.music_directory {
            return;
        }
        self.current_dir = self.config.directories.music_directory.clone();
        self.goto_top();
    }

    /// Lists all items in the directory; displays directories as their name, files as their metadata name, and both by their respective colors.
    pub fn list_items(&self) -> Vec<ListItem<'_>> {
        let dir_style = Style::default().fg(Color::from_str(&self.config.colors.fs_directory)
            .expect("If the color is set correctly in the config then this shouldn't fail"));
        let file_style = Style::default().fg(Color::from_str(&self.config.colors.fs_file)
            .expect("If the color is set correctly in the config then this shouldn't fail"));

        self.entries
            .iter()
            .map(|entry| {
                let (display_name, style) = if entry.is_dir() {
                    (
                        entry
                            .file_name()
                            .map(|s| format!("[{}]", s.to_string_lossy()))
                            .unwrap_or("Unknown".to_string()),
                        dir_style,
                    )
                } else {
                    let file_data = FileMetadata::get_file_data(entry);
                    (file_data.title.unwrap_or(file_data.raw_file), file_style)
                };
                ListItem::new(display_name).style(style)
            })
            .collect()
    }
}
