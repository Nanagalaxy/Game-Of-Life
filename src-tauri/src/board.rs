use crate::cell::Cell;
use dashmap::DashMap;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
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
    /// XXX: This function is not used
    fn _get_generation(&self) -> usize {
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
    pub fn get_cell(&self, id: Uuid) -> Option<Arc<Cell>> {
        self.cells
            .get(&id)
            .map(|cell_ref| Arc::clone(&cell_ref.value()))
    }

    /// Remove a cell from the board by its id
    /// XXX: This function is not used
    fn _remove_cell(&self, id: Uuid) {
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

    /// Get a list of all cells on the board
    pub fn get_cells(&self) -> Vec<Arc<Cell>> {
        self.cells
            .par_iter()
            .map(|cell| Arc::clone(&cell.value()))
            .collect()
    }

    /// Find a cell on the board by its position
    /// XXX: This function is not used
    fn _find_cell(&self, x: usize, y: usize) -> Option<Arc<Cell>> {
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

    /// Kill all cells on the board and reset the generation to 0
    pub fn kill_board(&self) {
        self.cells
            .par_iter()
            .for_each(|cell| cell.value().set_alive(false));

        self.reset_generation();
    }

    /// Compute the next generation of the board
    /// Returns a list of cell ids with their future state
    pub fn compute_next_generation(&self) -> Vec<(Uuid, bool)> {
        let relevant_cells = self.get_relevant_cells();

        relevant_cells
            .par_iter()
            .map(|cell| {
                let cell = cell.value();

                let alive = cell.compute_future_state();

                (cell.id, alive)
            })
            .collect()
    }

    /// Get the relevant cells for the next generation.
    /// A cell is relevant if it is alive or is a neighbor of an alive cell.
    fn get_relevant_cells(&self) -> DashMap<Uuid, Arc<Cell>> {
        let relevant_cells = DashMap::new();

        self.cells
            .par_iter()
            // Filter out the dead cells
            .filter(|entry| *entry.value().alive.lock().unwrap())
            .for_each(|entry| {
                let cell = entry.value();
                let cell_id = entry.key();

                // Add the alive cell to the relevant cells
                relevant_cells.insert(*cell_id, Arc::clone(&cell));

                // Add the neighbors of the alive cell to the relevant cells
                let neighbors = cell.get_neighbors();

                for neighbor in neighbors {
                    relevant_cells
                        .entry(neighbor.id)
                        .or_insert_with(|| Arc::clone(&neighbor));
                }
            });

        relevant_cells
    }

    /// Update the next generation of the board with the given list of cell ids and their future state
    pub fn update_next_generation(&self, next_gen: &Vec<(Uuid, bool)>) {
        next_gen.par_iter().for_each(|(id, alive)| {
            if let Some(cell) = self.get_cell(*id) {
                cell.set_alive(*alive);
            }
        });

        self.increment_generation();
    }
}
