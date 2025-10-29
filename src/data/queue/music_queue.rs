use crate::data::{
    config::{ConfigData, load_config},
    metadata::file_metadata::FileMetadata,
};
use ratatui::{
    style::{Color, Style},
    widgets::{ListItem, ListState},
};
use std::{io::Result, path::PathBuf};

pub struct MusicQueue {
    pub config: ConfigData,
    pub entries: Vec<PathBuf>,
    pub list_state: ListState,
    pub selected: usize,
}

impl MusicQueue {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            config: load_config(),
            entries: Vec::new(),
            list_state,
            selected: 0,
        }
    }

    pub fn update_entries(&mut self) -> Result<()> {
        self.list_state.select(match self.entries.is_empty() {
            true => None,
            false => Some(self.selected),
        });
        Ok(())
    }

    pub fn list_items(&self) -> Vec<ListItem<'_>> {
        self.entries
            .iter()
            .map(|entry| {
                let name = FileMetadata::get_file_data(entry);
                let display = name.title.unwrap_or(name.raw_file);
                let style = Style::default().fg(Color::White);

                ListItem::new(display).style(style)
            })
            .collect()
    }
}
