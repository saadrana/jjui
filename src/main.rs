mod app;
mod error;
mod tui;

fn main() -> Result<(), anyhow::Error> {
    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?; // ratatui terminal

    // application state
    let mut model = app::Model::default();

    while model.running_state != app::RunningState::Done {
        // Render the current view
        terminal.draw(|frame| app::view(&mut model, frame))?;

        // Handle events and map to a Message
        let mut current_msg = app::handle_event(&model)?;

        // Process updates as long as they return a non-None message
        while current_msg.is_some() {
            current_msg = app::update(&mut model, current_msg.unwrap());
        }
    }

    tui::restore_terminal()?;
    Ok(())
}
