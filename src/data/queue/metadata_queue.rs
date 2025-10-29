use ratatui::{
    style::{Color, Style},
    widgets::ListItem,
};

use crate::data::metadata::file_metadata::FileMetadata;
use std::{io::Result, path::PathBuf};

/// Encapsulates metadata queue information for correct displaying.
#[derive(Clone)]
pub struct MetadataQueue {
    pub current: FileMetadata,
    pub queue: Vec<FileMetadata>,
    // pub list: Vec<ListItem>,
}

impl MetadataQueue {
    pub fn new() -> Self {
        Self {
            current: FileMetadata::new(),
            queue: Vec::new(),
        }
    }

    // pub fn update_entries(&mut self) -> Result<()> {
    //     self.list_state.select(match self.queue.is_empty() {
    //         true => None,
    //         false => Some(self.selected),
    //     });
    //     Ok(())
    // }

    pub fn list_items(&self) -> Vec<ListItem<'_>> {
        self.queue
            .iter()
            .map(|entry| {
                let name = if let Some(title) = entry.clone().title {
                    title
                } else {
                    entry.clone().raw_file
                };

                let style = Style::default().fg(Color::White);
                ListItem::new(name).style(style)
            })
            .collect()
    }

    /// Updates the current metadata.
    pub fn update_current(&mut self, path: &PathBuf, clear: bool) {
        let data = FileMetadata::get_file_data(path);
        if clear {
            self.queue.clear();
        }
        self.queue.insert(0, data.clone());
        self.current = data;
    }

    /// Appends metadata for a queued song.
    pub fn queue_metadata(&mut self, path: &PathBuf) {
        self.queue.push(FileMetadata::get_file_data(path));
    }

    /// When skipping, remove the current metadata (index 0), set it to the next in the vec, then update current.
    pub fn pop_next(&mut self) -> Option<FileMetadata> {
        if !self.queue.is_empty() {
            self.queue.remove(0);
        }
        match self.queue.first() {
            Some(next) => {
                self.current = next.clone();
                Some(next.clone())
            }
            None => {
                self.current = FileMetadata::new();
                None
            }
        }
    }
}
