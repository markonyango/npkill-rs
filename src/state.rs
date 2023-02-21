use tui::widgets::TableState;

use crate::dirs::NodeModuleEntry;

#[derive(Default, Debug)]
pub struct AppState {
    pub list_state: TableState,
    pub dirs: Vec<NodeModuleEntry>,
    pub loading: bool,
}

impl AppState {
    pub fn new() -> Self {
        let mut state = Self::default();

        state.list_state.select(Some(0));
        state.loading = true;

        state
    }
}
