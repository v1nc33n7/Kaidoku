use rand::{seq::SliceRandom, thread_rng};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Cell {
    pub vector: (usize, usize),
    pub is_collapsed: bool,
    pub is_edited: bool,
    pub options: Vec<usize>,
}

impl Cell {
    fn new(vector: (usize, usize)) -> Self {
        Cell {
            vector,
            is_collapsed: false,
            is_edited: false,
            options: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        }
    }

    pub fn collapse_random(&mut self) -> Option<usize> {
        self.is_collapsed = true;

        let mut rng = thread_rng();
        let value = {
            let val_ref = self.options.choose(&mut rng)?;
            *val_ref
        };

        self.options = vec![value];
        Some(value)
    }

    pub fn collapse(&mut self, value: usize) {
        self.is_collapsed = true;
        self.options = vec![value];
    }
}

#[derive(Clone)]
pub struct Sudoku {
    pub dim: usize,
    pub grid_length: usize,
    pub cells: Vec<Cell>,
    pub saved_cells: Vec<Cell>,
    collapsed_on_column: HashMap<usize, Vec<usize>>,
    collapsed_on_row: HashMap<usize, Vec<usize>>,
    collapsed_on_grid: HashMap<(usize, usize), Vec<usize>>,
}

impl Sudoku {
    pub fn new(dim: usize, grid_length: usize) -> Self {
        let cells: Vec<Cell> = (0..dim)
            .flat_map(|y| (0..dim).map(move |x| Cell::new((x, y))))
            .collect();

        Self {
            dim,
            grid_length,
            cells,
            saved_cells: Vec::new(),
            collapsed_on_column: HashMap::default(),
            collapsed_on_row: HashMap::default(),
            collapsed_on_grid: HashMap::default(),
        }
    }

    pub fn save(&mut self) {
        self.saved_cells = self.cells.clone();
    }

    pub fn reset(&mut self) {
        self.collapsed_on_column = HashMap::default();
        self.collapsed_on_row = HashMap::default();
        self.collapsed_on_grid = HashMap::default();

        self.cells = self.saved_cells.clone();
    }

    pub fn add_on_axis(&mut self, row: usize, column: usize, collapsed_value: usize) {
        self.collapsed_on_row
            .entry(row)
            .or_insert(vec![collapsed_value])
            .push(collapsed_value);

        self.collapsed_on_column
            .entry(column)
            .or_insert(vec![collapsed_value])
            .push(collapsed_value);
    }

    pub fn remove_from_axis(&mut self, row: usize, column: usize, collapsed_value: usize) {
        if let Some(cells) = self.collapsed_on_row.get_mut(&row) {
            cells.retain(|&value| value != collapsed_value);
        }
        if let Some(cells) = self.collapsed_on_column.get_mut(&column) {
            cells.retain(|&value| value != collapsed_value);
        }
    }

    pub fn add_on_grid(&mut self, grid_index: (usize, usize), collapsed_value: usize) {
        self.collapsed_on_grid
            .entry(grid_index)
            .or_insert(vec![collapsed_value])
            .push(collapsed_value);
    }

    pub fn remove_from_grid(&mut self, grid_index: (usize, usize), collapsed_value: usize) {
        if let Some(cells) = self.collapsed_on_grid.get_mut(&grid_index) {
            cells.retain(|&value| value != collapsed_value);
        }
    }

    pub fn update(&mut self) {
        for row in 0..self.dim {
            for column in 0..self.dim {
                let index = column + row * self.dim;
                if let Some(cell) = self.cells.get_mut(index) {
                    if !cell.is_collapsed {
                        if let Some(collapsed_on_grid) = self
                            .collapsed_on_grid
                            .get(&(column / self.grid_length, row / self.grid_length))
                        {
                            cell.options.retain(|o| !collapsed_on_grid.contains(o));
                        }
                        if let Some(column) = self.collapsed_on_column.get(&column) {
                            cell.options.retain(|o| !column.contains(o));
                        }
                        if let Some(row) = self.collapsed_on_row.get(&row) {
                            cell.options.retain(|o| !row.contains(o));
                        }
                    }
                }
            }
        }
    }

    pub fn choose_least_options(&mut self) -> Option<&mut Cell> {
        let min_options_len = self
            .cells
            .iter()
            .filter(|cell| !cell.is_collapsed)
            .map(|cell| cell.options.len())
            .min()?;

        let indices: Vec<usize> = self
            .cells
            .iter()
            .enumerate()
            .filter(|(_, cell)| !cell.is_collapsed && cell.options.len() == min_options_len)
            .map(|(idx, _)| idx)
            .collect();

        let mut rng = thread_rng();
        let &random_idx = indices.choose(&mut rng)?;

        self.cells.get_mut(random_idx)
    }

    pub fn set_cell_value(&mut self, row: usize, column: usize, value: usize) {
        let index = column + row * self.dim;
        let grid_index = (column / self.grid_length, row / self.grid_length);

        let is_collapsed;
        let previous_value;

        if let Some(cell) = self.cells.get_mut(index) {
            is_collapsed = cell.is_collapsed;
            previous_value = cell.options[0];
            cell.is_edited = true;
            cell.collapse(value);
        } else {
            return;
        }

        if is_collapsed {
            self.remove_from_axis(row, column, previous_value);
            self.remove_from_grid(grid_index, previous_value);
        }

        self.add_on_axis(row, column, value);
        self.add_on_grid(grid_index, value);
    }
}
