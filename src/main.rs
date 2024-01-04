mod error;
mod messages;
mod model;
mod tui;
mod update;
mod view;

use model::{Model, RunningState};
use tui::{init_terminal, install_panic_hook, restore_terminal};
use update::{handle_event, update};
use view::view;

fn main() -> Result<(), anyhow::Error> {
    install_panic_hook();
    let mut terminal = init_terminal()?; // ratatui terminal

    // application state
    let mut model = Model::default();

    while model.running_state != RunningState::Done {
        // Render the current view
        terminal.draw(|frame| view(&mut model, frame))?;

        // Handle events and map to a Message
        let mut current_msg = handle_event(&model)?;

        // Process updates as long as they return a non-None message
        while current_msg.is_some() {
            current_msg = update(&mut model, current_msg.unwrap());
        }
    }

    restore_terminal()?;
    Ok(())
}
