
use tui::widgets::ListState;

/// A Stateful list that encapsulates the tui ListState Widget
#[derive(Debug, Clone)]
pub struct StatefulList<T> {
    /// The tui ListState Widget
    pub state: ListState,
    /// Inner items
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    /// Creates a new `StatefulList` with the given items.
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    /// Push an item into the list
    pub fn push_item(&mut self, item: T) {
        self.items.push(item);
    }

    /// Get the next item in the list
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    /// Get the previous item in the list
    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}