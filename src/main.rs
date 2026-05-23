mod core;

use crate::core::AppState::AppState;

fn main() {
    let mut appState = AppState::default();
    if !appState.updateFromCli() {
        return;
    }
}
