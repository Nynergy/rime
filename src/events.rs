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
            AppState::TagEditor => handle_tag_editor_events(key, app),
        }
    }

    Ok(())
}

fn handle_tag_editor_events(key: KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Char('q') => app.quit = true,
        KeyCode::Esc => app.quit = true,
        _ => {}
    }
}
