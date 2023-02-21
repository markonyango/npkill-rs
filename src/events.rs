use std::{error::Error, io::Stdout, sync::mpsc::Receiver};

use crossterm::{
    event::{KeyCode, KeyEvent, KeyEventKind},
    terminal::disable_raw_mode,
};
use tui::{backend::CrosstermBackend, Terminal};

use crate::{dirs::NodeModuleEntry, AppState};

pub enum Event<I> {
    Input(I),
    Tick,
    ListData(Vec<NodeModuleEntry>),
}

pub fn handle_event(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    rx: &Receiver<Event<KeyEvent>>,
    state: &mut AppState,
) -> Result<(), Box<dyn Error>> {
    match rx.recv()? {
        Event::Input(event) => match event.code {
            KeyCode::Char('q') => {
                disable_raw_mode()?;
                terminal.show_cursor()?;
                terminal.clear()?;
                terminal.set_cursor(0, 0)?;
                std::process::exit(0x0000)
            }
            KeyCode::Down if event.kind == KeyEventKind::Press => {
                if let Some(selected) = state.list_state.selected() {
                    let amount_dirs = state.dirs.len();

                    if selected > amount_dirs - 1 {
                        state.list_state.select(Some(0));
                    } else {
                        state.list_state.select(Some(selected + 1));
                    }
                }
            }
            KeyCode::Up if event.kind == KeyEventKind::Press => {
                if let Some(selected) = state.list_state.selected() {
                    let amount_dirs = state.dirs.len();
                    if selected > 0 {
                        state.list_state.select(Some(selected - 1));
                    } else {
                        state.list_state.select(Some(amount_dirs - 1));
                    }
                }
            }
            _ => {}
        },
        Event::Tick => {}
        Event::ListData(list_data) => {
            state.dirs = list_data;
            state.loading = false;
        }
    }

    Ok(())
}
