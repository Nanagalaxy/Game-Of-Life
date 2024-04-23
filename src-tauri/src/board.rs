use crate::cell::Cell;
use dashmap::DashMap;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug)]
pub struct Board {
    width: Mutex<usize>,
    height: Mutex<usize>,
    generation: Mutex<usize>,
    cells: DashMap<Uuid, Arc<Cell>>,
    position_to_id: DashMap<(usize, usize), Uuid>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            width: Mutex::new(0),
            height: Mutex::new(0),
            generation: Mutex::new(0),
            cells: DashMap::new(),
            position_to_id: DashMap::new(),
        }
    }

    fn set_size(&self, width: usize, height: usize) {
        let mut board_width = self.width.lock().unwrap();
        let mut board_height = self.height.lock().unwrap();
        *board_width = width;
        *board_height = height;
    }

    fn increment_generation(&self) {
        let mut generation = self.generation.lock().unwrap();
        *generation += 1;
    }

    fn get_generation(&self) -> usize {
        *self.generation.lock().unwrap()
    }

    fn reset_generation(&self) {
        let mut generation = self.generation.lock().unwrap();
        *generation = 0;
    }

    fn add_cell(&self, cell: Arc<Cell>) {
        self.cells.insert(cell.id, cell);
    }

    fn get_cell(&self, id: Uuid) -> Option<Arc<Cell>> {
        self.cells
            .get(&id)
            .map(|cell_ref| Arc::clone(&cell_ref.value()))
    }

    fn remove_cell(&self, id: Uuid) {
        self.cells.remove(&id);
    }

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

    fn clear_cells(&self) {
        self.cells.clear();
    }

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

    fn reset(&self) {
        self.reset_generation();
        self.clear_cells();
    }

    pub fn create_board(&self, width: usize, height: usize) {
        self.reset();

        self.set_size(width, height);

        self.fill_cells();

        self.compute_neighbors();
    }
}
