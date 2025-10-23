use crate::{
    data::{
        config::{ConfigData, load_config},
        metadata::{file_metadata::FileMetadata, metadata_queue::MetadataQueue},
    },
    handlers::input_handler::InputHandler,
    tui::fs_browser::FileBrowser,
};
use crossterm::event::poll;
use ratatui::DefaultTerminal;
use std::{
    error::Error,
    path::PathBuf,
    thread::sleep,
    time::{Duration, Instant},
};

/// The main application.
pub struct App {
    pub config: ConfigData,
    pub meta_manager: MetadataQueue,
    pub file_browser: FileBrowser,
    pub audio: InputHandler,
    pub data: FileMetadata,
    pub path_queue: Vec<PathBuf>,
    pub prog_bar: f64,
    pub tab: Tab,
    pub state: State,
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
        const RENDER_INTERVAL: Duration = Duration::from_millis(1000 / 60);

        let mut last_render = Instant::now();
        let mut last_progbar_update = Instant::now();

        while self.state == State::Running {
            let loop_start = Instant::now();

            if poll(Duration::from_millis(10))? {
                self.handle_events()?;
            }

            if !self.audio.is_empty()
                && loop_start.duration_since(last_progbar_update) >= Duration::from_millis(50)
            {
                self.update_prog_bar();
                last_progbar_update = loop_start;
            }

            self.file_browser.update_entries()?;

            if loop_start.duration_since(last_render) >= RENDER_INTERVAL {
                terminal.draw(|frame| self.draw(frame))?;
                last_render = loop_start;
            } else {
                sleep(Duration::from_millis(1));
            }
        }
        Ok(())
    }
}
