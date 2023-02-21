use std::{
    env::{self},
    error::Error,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    event::{self, EnableMouseCapture, Event as CEvent, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};
use dirs::{get_dirs, NodeModuleEntry};
use events::{handle_event, Event};
use tui::{backend::CrosstermBackend, widgets::TableState, Terminal};
use ui::ui;

mod dirs;
mod error;
mod events;
mod prelude;
mod ui;
mod utils;

#[derive(Default, Debug)]
pub struct AppState {
    pub list_state: TableState,
    pub dirs: Vec<NodeModuleEntry>,
    pub loading: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut app_state = AppState::default();
    app_state.list_state.select(Some(0));
    app_state.loading = true;
    let args = env::args().next();
    let path = match args {
        Some(arg) => arg,
        None => String::from("."),
    };

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);

    let input_tx = tx.clone();
    let list_data_tx = tx.clone();

    thread::spawn(move || {
        let mut last_tick = Instant::now();

        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    input_tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate && tx.send(Event::Tick).is_ok() {
                last_tick = Instant::now();
            }
        }
    });

    thread::spawn(move || {
        let dirs = get_dirs(&path);

        list_data_tx
            .send(Event::ListData(dirs))
            .expect("Couldn't traverse directories.");
    });

    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    enable_raw_mode()?;

    execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    loop {
        terminal.draw(|f| ui(f, &mut app_state))?;

        handle_event(&mut terminal, &rx, &mut app_state);

        // match rx.recv()? {
        //     Event::Input(event) => match event.code {
        //         KeyCode::Char('q') => {
        //             disable_raw_mode()?;
        //             terminal.show_cursor()?;
        //             terminal.clear()?;
        //             terminal.set_cursor(0, 0)?;
        //             break;
        //         }
        //         KeyCode::Down if event.kind == KeyEventKind::Press => {
        //             if let Some(selected) = app_state.list_state.selected() {
        //                 let amount_dirs = app_state.dirs.len();

        //                 if selected > amount_dirs - 1 {
        //                     app_state.list_state.select(Some(0));
        //                 } else {
        //                     app_state.list_state.select(Some(selected + 1));
        //                 }
        //             }
        //         }
        //         KeyCode::Up if event.kind == KeyEventKind::Press => {
        //             if let Some(selected) = app_state.list_state.selected() {
        //                 let amount_dirs = app_state.dirs.len();
        //                 if selected > 0 {
        //                     app_state.list_state.select(Some(selected - 1));
        //                 } else {
        //                     app_state.list_state.select(Some(amount_dirs - 1));
        //                 }
        //             }
        //         }
        //         _ => {}
        //     },
        //     Event::Tick => {}
        //     Event::ListData(list_data) => {
        //         app_state.dirs = list_data;
        //         app_state.loading = false;
        //     }
        // }
    }

    Ok(())
}

// pub enum Event<I> {
//     Input(I),
//     Tick,
//     ListData(Vec<NodeModuleEntry>),
// }
