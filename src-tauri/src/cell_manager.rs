pub mod cell {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[readonly::make]
    pub struct Cell {
        #[readonly]
        pub id: Uuid,

        pub alive: bool,

        #[readonly]
        pub x: u32,

        #[readonly]
        pub y: u32,
    }

    impl Cell {
        pub fn new(alive: bool, x: u32, y: u32) -> Self {
            Self {
                id: Uuid::new_v4(),
                alive,
                x,
                y,
            }
        }

        /// Toggle the alive state of the cell
        ///
        /// # Examples
        /// ```rust
        /// use cell_manager::cell::Cell;
        ///
        /// let mut cell = Cell::new(true, 0, 0);
        ///
        /// cell.toggle_alive();
        ///
        /// assert_eq!(cell.alive, false);
        ///
        /// cell.toggle_alive();
        ///
        /// assert_eq!(cell.alive, true);
        /// ```
        pub fn toggle_alive(&mut self) {
            self.alive = !self.alive;
        }

        /// Set the alive state of the cell
        ///
        /// # Examples
        /// ```rust
        /// use cell_manager::cell::Cell;
        ///
        /// let mut cell = Cell::new(true, 0, 0);
        ///
        /// cell.set_alive(false);
        ///
        /// assert_eq!(cell.alive, false);
        /// ```
        pub fn set_alive(&mut self, alive: bool) {
            self.alive = alive;
        }

        /// Find a cell by its ID
        ///
        /// # Examples
        /// ```rust
        /// use cell_manager::cell::Cell;
        /// use uuid::Uuid;
        ///
        /// let cell = Cell::new(true, 0, 0);
        ///
        /// let id = cell.id;
        ///
        /// let cells = vec![
        ///    cell,
        /// ];
        ///
        /// let cell = Cell::find_by_id(cells, id);
        ///
        /// assert_eq!(cell.is_some(), true);
        /// ```
        pub fn find_by_id(cells: Vec<Cell>, id: Uuid) -> Option<Cell> {
            for cell in cells {
                if cell.id == id {
                    return Some(cell);
                }
            }

            None
        }

        /// Find a cell by its position
        ///
        /// # Examples
        /// ```rust
        /// use cell_manager::cell::Cell;
        ///
        /// let cells = vec![
        ///    Cell::new(true, 0, 0),
        /// ];
        ///
        /// let cell = Cell::find_by_position(cells, 0, 0);
        ///
        /// assert_eq!(cell.is_some(), true);
        /// ```
        pub fn find_by_position(cells: Vec<Cell>, x: u32, y: u32) -> Option<Cell> {
            for cell in cells {
                if cell.x == x && cell.y == y {
                    return Some(cell);
                }
            }

            None
        }

        /// Offset the position by the given offset
        ///
        /// # Examples
        /// ```rust
        /// use cell_manager::cell::Cell;
        ///
        /// let x = 0;
        /// let offset = 1;
        ///
        /// let new_x = Cell::offset_position(x, offset);
        ///
        /// assert_eq!(new_x, Some(1));
        ///
        /// let x = 0;
        /// let offset = -1;
        ///
        /// let new_x = Cell::offset_position(x, offset);
        ///
        /// assert_eq!(new_x, None); // Negative position is not allowed
        ///
        /// let x = 1;
        /// let offset = -1;
        ///
        /// let new_x = Cell::offset_position(x, offset);
        ///
        /// assert_eq!(new_x, Some(0)); // Negative offset is allowed
        /// ```
        pub fn offset_position(position: u32, offset: i8) -> Option<u32> {
            if offset < 0 {
                position.checked_sub(offset.abs() as u32)
            } else {
                position.checked_add(offset as u32)
            }
        }

        /// Get the neighbors of the cell
        ///
        /// # Examples
        /// ```rust
        /// use cell_manager::cell::Cell;
        ///
        /// let cell = Cell::new(true, 0, 0);
        ///
        /// let cells = vec![
        ///    Cell::new(true, 0, 1), // South
        ///    Cell::new(true, 1, 0), // East
        ///    Cell::new(true, 1, 1), // South East
        ///    Cell::new(true, 1, -1), // North East
        /// ];
        ///
        /// let neighbors = cell.get_neighbors(cells);
        /// // neighbors contains the cells at (0, 1), (1, 0), (1, 1)
        ///
        /// assert_eq!(neighbors.len(), 3);
        /// ```
        pub fn neighbors(&self, cells: Vec<Cell>) -> Vec<Cell> {
            let mut neighbors = Vec::new();

            for (dx, dy) in NeighborsPosition::get_all() {
                let offset_x = if let Some(offset_x) = Self::offset_position(self.x, dx) {
                    offset_x
                } else {
                    continue;
                };
                let offset_y = if let Some(offset_y) = Self::offset_position(self.y, dy) {
                    offset_y
                } else {
                    continue;
                };

                if let Some(neighbor) = Cell::find_by_position(cells.clone(), offset_x, offset_y) {
                    neighbors.push(neighbor);
                }
            }

            neighbors
        }

        pub fn alive_neighbors(&self, cells: Vec<Cell>) -> Vec<Cell> {
            self.neighbors(cells)
                .into_iter()
                .filter(|cell| cell.alive)
                .collect()
        }

        pub fn dead_neighbors(&self, cells: Vec<Cell>) -> Vec<Cell> {
            self.neighbors(cells)
                .into_iter()
                .filter(|cell| !cell.alive)
                .collect()
        }
    }

    struct NeighborsPosition;

    impl NeighborsPosition {
        const NORTH_WEST: (i8, i8) = (-1, -1);
        const NORTH: (i8, i8) = (0, -1);
        const NORTH_EAST: (i8, i8) = (1, -1);
        const WEST: (i8, i8) = (-1, 0);
        const EAST: (i8, i8) = (1, 0);
        const SOUTH_WEST: (i8, i8) = (-1, 1);
        const SOUTH: (i8, i8) = (0, 1);
        const SOUTH_EAST: (i8, i8) = (1, 1);

        fn get_all() -> Vec<(i8, i8)> {
            vec![
                Self::NORTH_WEST,
                Self::NORTH,
                Self::NORTH_EAST,
                Self::WEST,
                Self::EAST,
                Self::SOUTH_WEST,
                Self::SOUTH,
                Self::SOUTH_EAST,
            ]
        }
    }
}

use self::cell::Cell;

fn cell_future_state(cell: Cell, neighbors: Vec<Cell>) -> bool {
    let alive_neighbors = cell.alive_neighbors(neighbors.clone());

    if cell.alive {
        match alive_neighbors.len() {
            2 | 3 => true,
            _ => false,
        }
    } else {
        match alive_neighbors.len() {
            3 => true,
            _ => false,
        }
    }
}

pub fn compute_next_generation(cells: Vec<Cell>) -> Vec<Cell> {
    let mut next_generation = Vec::new();

    for cell in &cells {
        let neighbors = cell.neighbors(cells.clone());
        let alive = cell_future_state(cell.clone(), neighbors);

        if alive != cell.alive {
            let mut cell = cell.clone();
            cell.set_alive(alive);
            next_generation.push(cell);
        } else {
            next_generation.push(cell.clone());
        }
    }

    next_generation
}
