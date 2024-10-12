pub enum SudokuStatus {
    Running,
    Success,
    Fail,
}

#[derive(PartialEq)]
pub enum CurrentScreen {
    Editing,
    Running,
    Finish,
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub selected_row: usize,
    pub selected_column: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_screen: CurrentScreen::Editing,
            selected_row: 0,
            selected_column: 0,
        }
    }
}
