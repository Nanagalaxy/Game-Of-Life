use crate::cell::Cell;
use dashmap::DashMap;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// A struct representing a board
#[derive(Debug)]
pub struct Board {
    /// The width of the board
    width: Mutex<usize>,

    /// The height of the board
    height: Mutex<usize>,

    /// The generation of the board
    generation: Mutex<usize>,

    /// The list of cells on the board
    cells: DashMap<Uuid, Arc<Cell>>,

    /// A map of cell positions to cell ids
    position_to_id: DashMap<(usize, usize), Uuid>,
}

impl Board {
    /// Create a new board
    pub fn new() -> Self {
        Self {
            width: Mutex::new(0),
            height: Mutex::new(0),
            generation: Mutex::new(0),
            cells: DashMap::new(),
            position_to_id: DashMap::new(),
        }
    }

    /// Set the size of the board
    fn set_size(&self, width: usize, height: usize) {
        let mut board_width = self.width.lock().unwrap();
        let mut board_height = self.height.lock().unwrap();
        *board_width = width;
        *board_height = height;
    }

    /// Increment the generation of the board by 1
    fn increment_generation(&self) {
        let mut generation = self.generation.lock().unwrap();
        *generation += 1;
    }

    /// Get the current generation of the board
    fn get_generation(&self) -> usize {
        *self.generation.lock().unwrap()
    }

    /// Reset the generation of the board to 0
    fn reset_generation(&self) {
        let mut generation = self.generation.lock().unwrap();
        *generation = 0;
    }

    /// Add a cell to the board
    fn add_cell(&self, cell: Arc<Cell>) {
        self.cells.insert(cell.id, cell);
    }

    /// Get a cell from the board by its id
    fn get_cell(&self, id: Uuid) -> Option<Arc<Cell>> {
        self.cells
            .get(&id)
            .map(|cell_ref| Arc::clone(&cell_ref.value()))
    }

    /// Remove a cell from the board by its id
    fn remove_cell(&self, id: Uuid) {
        self.cells.remove(&id);
    }

    /// Fill the board with cells
    fn fill_cells(&self) {
        let width = *self.width.lock().unwrap();
        let height = *self.height.lock().unwrap();

        (0..width).into_par_iter().for_each(|x| {
            (0..height).into_par_iter().for_each(|y| {
                let new_cell = Cell::new(false, x, y);
                let id = new_cell.id;

                self.position_to_id.insert((x, y), id);

                self.add_cell(new_cell);
            });
        });
    }

    /// Find a cell on the board by its position
    fn find_cell(&self, x: usize, y: usize) -> Option<Arc<Cell>> {
        self.position_to_id
            .get(&(x, y))
            .and_then(|id| self.get_cell(*id))
    }

    /// Clear all cells from the board
    fn clear_cells(&self) {
        self.cells.clear();
    }

    /// Compute the neighbors of each cell on the board
    fn compute_neighbors(&self) {
        let width = *self.width.lock().unwrap();
        let height = *self.height.lock().unwrap();

        (0..width).into_par_iter().for_each(|x| {
            (0..height).into_par_iter().for_each(|y| {
                if let Some(cell_id) = self.position_to_id.get(&(x, y)) {
                    if let Some(cell) = self.cells.get(&cell_id) {
                        let neighbor_offsets = [
                            (-1, -1),
                            (0, -1),
                            (1, -1),
                            (-1, 0),
                            (1, 0),
                            (-1, 1),
                            (0, 1),
                            (1, 1),
                        ];

                        for &(dx, dy) in &neighbor_offsets {
                            if let (Some(nx), Some(ny)) =
                                (Cell::offset_position(x, dx), Cell::offset_position(y, dy))
                            {
                                if nx < width && ny < height {
                                    if let Some(neighbor_id) = self.position_to_id.get(&(nx, ny)) {
                                        if let Some(neighbor) = self.cells.get(&neighbor_id) {
                                            cell.add_neighbor(nx, ny, Arc::clone(&neighbor));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            });
        });
    }

    /// Reset the board to its initial state
    fn reset(&self) {
        self.reset_generation();
        self.clear_cells();
    }

    /// Create a new board with the given width and height, filling it with cells and computing the neighbors
    pub fn create_board(&self, width: usize, height: usize) {
        self.reset();

        self.set_size(width, height);

        self.fill_cells();

        self.compute_neighbors();
    }
}
