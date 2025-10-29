use crate::{
    data::metadata::file_metadata::FileMetadata,
    tui::app::{App, State, Tab},
};
use crossterm::event::{KeyCode, KeyEvent};

impl App {
    /// Handles key events.
    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        let vol_delta = self.config.controls.vol_delta;

        match key_event.code {
            KeyCode::Char('q') => self.state = State::Quit,

            KeyCode::Enter => self.handle_play(),
            KeyCode::Char('a') => self.handle_append(),
            KeyCode::Char('s') => self.handle_skip(),

            KeyCode::Up | KeyCode::Char('k') => self.file_browser.navigate_up(),
            KeyCode::Down | KeyCode::Char('j') => self.file_browser.navigate_down(),
            KeyCode::Left | KeyCode::Char('h') => self.file_browser.navigate_back(),
            KeyCode::Right | KeyCode::Char('l') => self.file_browser.navigate_into(),

            KeyCode::PageUp => self.file_browser.goto_top(),
            KeyCode::PageDown => self.file_browser.goto_bottom(),

            KeyCode::Char('g') => self.file_browser.goto_music_dir(),

            KeyCode::Char('c') => {
                self.audio.clear_sink();
                self.path_queue.clear();
                self.data = FileMetadata::new();
            }

            KeyCode::Char('=') | KeyCode::Char('+') => self.audio.adjust_volume(vol_delta),
            KeyCode::Char('-') | KeyCode::Char('_') => self.audio.adjust_volume(vol_delta * -1),
            KeyCode::Char('p') => self.audio.toggle_play_pause(),

            KeyCode::Char('1') => self.tab = Tab::Browser,
            KeyCode::Char('2') => self.tab = Tab::Playlist,

            _ => {}
        }
    }
}
