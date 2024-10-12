use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::Span,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, Wrap},
    Frame,
};

use crate::{app::App, CurrentScreen, Sudoku};

const SUDOKU_PADDING: u16 = 20;
const HEADER_HEIGHT: u16 = 2;
const FOOTER_HEIGHT: u16 = 4;
const SUDOKU_TITLE: &str = " Solve any Sudoku ";

pub fn ui(frame: &mut Frame, app: &mut App, sudoku: &Sudoku) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(HEADER_HEIGHT),
            Constraint::Min(1),
            Constraint::Length(FOOTER_HEIGHT),
        ])
        .split(frame.area());

    let frame_block = Block::default()
        .borders(Borders::ALL)
        .title(SUDOKU_TITLE)
        .title_alignment(Alignment::Center);
    frame.render_widget(frame_block, frame.area());

    let sudoku_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(0),
            Constraint::Max(main_layout[1].width - SUDOKU_PADDING),
            Constraint::Fill(0),
        ])
        .split(main_layout[1]);

    let table = Table::new(
        render_table(sudoku, app),
        grid_constraints(sudoku.dim, sudoku_layout[1]),
    );
    frame.render_widget(table, sudoku_layout[1]);

    let current_navigation_text = match app.current_screen {
        CurrentScreen::Editing => Span::styled("Editing Mode", Style::default()),
        CurrentScreen::Running => Span::styled("Running Mode", Style::default()),
        CurrentScreen::Finish => Span::styled("Finish Mode", Style::default()),
    };
    let mode_footer =
        Paragraph::new(current_navigation_text).block(Block::default().borders(Borders::ALL));

    let current_keys_hint = match app.current_screen {
        CurrentScreen::Editing => Span::styled(
            "(ARROWS) to navigate / (ENTER) to run / (r) to reset / (q) to quit",
            Style::default(),
        ),
        CurrentScreen::Running => Span::default(),
        CurrentScreen::Finish => Span::styled(
            "(r) to reset / (R) to run again / (q) to quit",
            Style::default(),
        ),
    };
    let key_notes_footer = Paragraph::new(current_keys_hint)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap::default());

    let footer_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50); 2])
        .split(main_layout[2]);

    frame.render_widget(mode_footer, footer_chunk[0]);
    frame.render_widget(key_notes_footer, footer_chunk[1]);
}

fn render_table<'a>(sudoku: &Sudoku, app: &App) -> Vec<Row<'a>> {
    (0..sudoku.dim)
        .map(|row| {
            (0..sudoku.dim)
                .map(|col| {
                    let idx = col + row * sudoku.dim;
                    match sudoku.cells.get(idx) {
                        Some(cell) => {
                            let color = determine_color(row, col);
                            let widget_cell = if cell.options.len() == 1 {
                                Cell::from(cell.options[0].to_string()).bg(color)
                            } else {
                                Cell::from("").bg(color)
                            };
                            let app_idx = app.selected_column + app.selected_row * sudoku.dim;
                            match (
                                cell.is_edited,
                                cell.is_collapsed,
                                idx == app_idx && app.current_screen == CurrentScreen::Editing,
                            ) {
                                (_, _, true) => widget_cell.on_yellow(),
                                (true, _, _) => widget_cell.black(),
                                (_, true, _) => widget_cell.white(),
                                _ => widget_cell,
                            }
                        }
                        None => Cell::from("Err"),
                    }
                })
                .collect()
        })
        .collect()
}

fn grid_constraints(sudoku_dim: usize, rect: Rect) -> Vec<Constraint> {
    (0..sudoku_dim)
        .map(|_| Constraint::Length(rect.width / sudoku_dim as u16))
        .collect()
}

fn determine_color(row: usize, col: usize) -> Color {
    if (row / 3 + col / 3) % 2 == 0 {
        Color::LightCyan
    } else {
        Color::Cyan
    }
}
