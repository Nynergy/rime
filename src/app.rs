pub enum AppState {
    TagEditor
}

pub struct App {
    pub state: AppState,
    pub quit: bool,
}

impl App {
    pub fn default() -> Self {
        Self {
            state: AppState::TagEditor,
            quit: false,
        }
    }

    pub fn on_tick(&mut self) {
        match self.state {
            AppState::TagEditor => {}
        }
    }
}
