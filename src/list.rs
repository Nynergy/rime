use tui::widgets::ListState;

pub struct GenericList<T> {
    pub items: Vec<T>,
    pub state: ListState,
}

impl<T: Clone> GenericList<T> {
    pub fn from(items: Vec<T>) -> Self {
        let mut state = ListState::default();
        if items.is_empty() {
            state.select(None);
        } else {
            state.select(Some(0));
        }

        Self { items, state }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn get_index(&self) -> Option<usize> {
        self.state.selected()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.state.select(index);
    }

    pub fn get_selected(&self) -> Option<T> {
        if let Some(i) = self.state.selected() {
            Some(self.items[i].clone())
        } else {
            None
        }
    }
}
