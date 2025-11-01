use crate::data::{
    config::{ConfigData, load_config},
    metadata::file_metadata::FileMetadata,
};
use ratatui::{
    style::{Color, Style},
    widgets::{ListItem, ListState},
};

pub struct MusicQueue {
    pub config: ConfigData,
    pub current: FileMetadata,
    pub list_state: ListState,
    pub queue: Vec<FileMetadata>,
}

impl MusicQueue {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            config: load_config(),
            current: FileMetadata::new(),
            list_state,
            queue: Vec::new(),
        }
    }

    pub fn list_items(&self) -> Vec<ListItem<'_>> {
        self.queue
            .iter()
            .map(|entry| {
                let data = FileMetadata::get_file_data(&entry.file_path);
                ListItem::new(data.title.unwrap_or(data.raw_file).clone())
                    .style(Style::default().fg(Color::White))
            })
            .collect()
    }
}
