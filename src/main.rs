use crate::tui::app::App;
use crossterm::{
    execute,
    terminal::{LeaveAlternateScreen, disable_raw_mode},
};
use std::{env, error::Error, io::stdout};

mod data;
mod handlers;
mod tui;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = ratatui::init();
    let current_dir = env::current_dir()?;
    let mut app = App::new(current_dir)?;
    let res = app.run(&mut terminal);
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    terminal.show_cursor()?;
    Ok(res?)
}
