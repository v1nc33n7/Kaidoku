use std::io;

use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::Backend, Terminal};

use app::*;
use sudoku::*;
use ui::*;

mod app;
mod sudoku;
mod ui;

fn main() {
    let mut terminal = ratatui::init();

    let mut app = App::new();
    let sudoku = Sudoku::new(9, 3);
    if let Err(e) = run_app(&mut terminal, &mut app, sudoku) {
        eprintln!("Application error: {}", e);
    }

    ratatui::restore();
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    mut sudoku: Sudoku,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app, &sudoku))?;

        match app.current_screen {
            CurrentScreen::Running => match loop_sudoku(&mut sudoku) {
                SudokuStatus::Running => continue,
                SudokuStatus::Fail => sudoku.reset(),
                SudokuStatus::Success => app.current_screen = CurrentScreen::Finish,
            },
            CurrentScreen::Editing => {
                if let Event::Key(key) = event::read()? {
                    if key.kind == event::KeyEventKind::Release {
                        continue;
                    }

                    match key.code {
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        KeyCode::Char('r') => {
                            sudoku = Sudoku::new(sudoku.dim, sudoku.grid_length);
                        }

                        KeyCode::Up => {
                            if app.selected_row == 0 {
                                continue;
                            }
                            app.selected_row -= 1;
                        }
                        KeyCode::Down => {
                            if app.selected_row == sudoku.dim - 1 {
                                continue;
                            }
                            app.selected_row += 1;
                        }
                        KeyCode::Left => {
                            if app.selected_column == 0 {
                                continue;
                            }
                            app.selected_column -= 1;
                        }
                        KeyCode::Right => {
                            if app.selected_column == sudoku.dim - 1 {
                                continue;
                            }
                            app.selected_column += 1;
                        }
                        KeyCode::Char(c) => {
                            if let Some(num) = c.to_digit(10) {
                                sudoku.set_cell_value(
                                    app.selected_row,
                                    app.selected_column,
                                    num as usize,
                                );
                            }
                        }
                        KeyCode::Enter => {
                            sudoku.save();
                            app.current_screen = CurrentScreen::Running;
                        }
                        _ => {}
                    }
                }
            }
            CurrentScreen::Finish => {
                if let Event::Key(key) = event::read()? {
                    if key.kind == event::KeyEventKind::Release {
                        continue;
                    }

                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('r') => {
                            sudoku = Sudoku::new(sudoku.dim, sudoku.grid_length);
                            app.current_screen = CurrentScreen::Editing;
                        }
                        KeyCode::Char('R') => {
                            sudoku.reset();
                            app.current_screen = CurrentScreen::Running;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn loop_sudoku(sudoku: &mut Sudoku) -> SudokuStatus {
    match sudoku.choose_least_options() {
        Some(cell) => match cell.collapse_random() {
            Some(option) => {
                let vector = cell.vector;
                sudoku.add_on_axis(vector.1, vector.0, option);
                sudoku.add_on_grid(
                    (vector.0 / sudoku.grid_length, vector.1 / sudoku.grid_length),
                    option,
                );
                sudoku.update();
                SudokuStatus::Running
            }
            None => SudokuStatus::Fail,
        },
        None => SudokuStatus::Success,
    }
}
