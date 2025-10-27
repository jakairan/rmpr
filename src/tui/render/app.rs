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
    pub prog_bar: f64,
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
        let music_dir = load_config().directories.music_directory;

        let final_dir = match music_dir.exists() {
            true => music_dir,
            false => initial_dir,
        };

        Ok(Self {
            config: load_config(),
            meta_manager: MetadataQueue::new(),
            file_browser: FileBrowser::new(final_dir),
            audio: InputHandler::new()?,
            data: FileMetadata::new(),
            path_queue: Vec::new(),
            prog_bar: 0.0,
            tab: Tab::Browser,
            state: State::Running,
        })
    }

    /// Update's the progress bar's apperance.
    ///
    /// Displays in milliseconds / milliseconds for higher resolution seekbar.Originally intended for gauge's use_unicode(), but it's being kept in case I decide to go back to gauge.
    pub fn update_prog_bar(&mut self) {
        if self.audio.is_empty() {
            self.prog_bar = 0.0;
            return;
        }
        self.prog_bar = (self.audio.sink_pos_millis() as f64
            / (self.data.duration_as_secs.unwrap() * 1000.0))
            .clamp(0.0, 1.0);
    }

    /// Renders the tui.
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        self.file_browser.update_entries()?;
        let tick_rate = Duration::from_secs(1); // renders at 1fps
        let mut last_tick = Instant::now();

        while self.state == State::Running {
            terminal.draw(|frame| self.render(frame))?;
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if !event::poll(timeout)? {
                self.update_prog_bar();
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
