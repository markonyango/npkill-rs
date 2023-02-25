use std::{error::Error, sync::mpsc, thread, time::Duration};

use args::Cli;
use clap::Parser;
use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen},
};

use tui::{backend::CrosstermBackend, Terminal};

mod dirs;
mod error;
mod events;
mod prelude;
mod state;
mod ui;
mod utils;
mod args;

fn main() -> Result<(), Box<dyn Error>> {
    let Cli { path } = Cli::parse();
    
    let mut app_state = state::AppState::new();

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);

    let input_tx = tx.clone();
    let list_data_tx = tx;

    thread::spawn(move || events::handle_key_event(tick_rate, input_tx));

    thread::spawn(move || events::handle_dir_event(&path, list_data_tx));

    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    enable_raw_mode()?;

    execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    loop {
        terminal.draw(|f| ui::ui(f, &mut app_state))?;

        events::handle_event(&mut terminal, &rx, &mut app_state)?;
    }
}
