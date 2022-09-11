use crossterm::{
    event::{
        self,
        Event,
        KeyCode,
        KeyEvent
    },
};
use std::error::Error;

use crate::app::*;

type DynResult<T> = Result<T, Box<dyn Error>>;

pub fn handle_events(app: &mut App) -> DynResult<()> {
    if let Event::Key(key) = event::read()? {
        match app.state {
            AppState::FileNavigation =>
                handle_file_navigation_events(key, app)?,
        }
    }

    Ok(())
}

fn handle_file_navigation_events(
    key: KeyEvent,
    app: &mut App
) -> DynResult<()> {
    match key.code {
        KeyCode::Char('q') => app.quit = true,
        KeyCode::Esc => app.quit = true,
        KeyCode::Char('h') => app.exit_dir()?,
        KeyCode::Char('l') => app.enter_dir()?,
        KeyCode::Char('j') => app.list_down(),
        KeyCode::Down => app.list_down(),
        KeyCode::Char('k') => app.list_up(),
        KeyCode::Up => app.list_up(),
        KeyCode::Char('g') => app.jump_to_list_top(),
        KeyCode::Home => app.jump_to_list_top(),
        KeyCode::Char('G') => app.jump_to_list_bottom(),
        KeyCode::End => app.jump_to_list_bottom(),
        KeyCode::Char(' ') => app.select()?,
        KeyCode::Enter => app.select()?,
        _ => {}
    }

    Ok(())
}
