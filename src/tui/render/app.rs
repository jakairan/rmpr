use crate::{
    data::{
        config::{ConfigData, load_config},
        metadata::{file_metadata::FileMetadata, metadata_queue::MetadataQueue},
    },
    handlers::input_handler::InputHandler,
    tui::fs_browser::FileBrowser,
};
use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;
use std::{
    error::Error,
    path::PathBuf,
    time::{Duration, Instant},
};

/// The main application.
pub struct App {
    pub audio: InputHandler,
    pub config: ConfigData,
    pub data: FileMetadata,
    pub file_browser: FileBrowser,
    pub meta_manager: MetadataQueue,
    pub path_queue: Vec<PathBuf>,
    pub state: State,
    pub tab: Tab,
}
/// Current tab information.
pub enum Tab {
    Playlist,
    Browser,
}

/// App state.
#[derive(PartialEq)]
pub enum State {
    Running,
    Quit,
}

impl App {
    pub fn new(initial_dir: PathBuf) -> Result<Self, Box<dyn Error>> {
        let final_dir = dirs::home_dir()
            .map(|mut path| {
                path.push("Music");
                path
            })
            .unwrap_or(initial_dir);

        Ok(Self {
            config: load_config(),
            meta_manager: MetadataQueue::new(),
            file_browser: FileBrowser::new(final_dir),
            audio: InputHandler::new()?,
            data: FileMetadata::new(),
            path_queue: Vec::new(),
            tab: Tab::Browser,
            state: State::Running,
        })
    }

    /// Renders the tui.
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        self.file_browser.update_entries()?;
        let tick_rate = Duration::from_millis(100); // renders at 10fps
        let mut last_tick = Instant::now();

        while self.state == State::Running {
            terminal.draw(|frame| self.render(frame))?;
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if !event::poll(timeout)? {
                last_tick = Instant::now();
                continue;
            }
            if let Event::Key(key) = event::read()? {
                self.handle_key_event(key);
                self.file_browser.update_entries()?;
            }
        }
        Ok(())
    }
}
