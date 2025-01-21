use wasm_bindgen::prelude::*;
use js_sys::Math;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    generation: u32,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        let mut cells = vec![Cell::Dead; (width * height) as usize];
        
        // Add many more spaceships and gliders
        for _ in 0..100 {  // Increased to 100 patterns
            let x = (Math::random() * (width as f64)) as u32;
            let y = (Math::random() * (height as f64)) as u32;
            
            if x + 5 < width && y + 5 < height {
                match (Math::random() * 6.0) as i32 {
                    0 => { // Glider
                        cells[((y + 0) * width + (x + 1)) as usize] = Cell::Alive;
                        cells[((y + 1) * width + (x + 2)) as usize] = Cell::Alive;
                        cells[((y + 2) * width + (x + 0)) as usize] = Cell::Alive;
                        cells[((y + 2) * width + (x + 1)) as usize] = Cell::Alive;
                        cells[((y + 2) * width + (x + 2)) as usize] = Cell::Alive;
                    },
                    1 => { // Lightweight spaceship
                        cells[((y + 0) * width + (x + 1)) as usize] = Cell::Alive;
                        cells[((y + 0) * width + (x + 2)) as usize] = Cell::Alive;
                        cells[((y + 0) * width + (x + 3)) as usize] = Cell::Alive;
                        cells[((y + 0) * width + (x + 4)) as usize] = Cell::Alive;
                        cells[((y + 1) * width + (x + 0)) as usize] = Cell::Alive;
                        cells[((y + 1) * width + (x + 4)) as usize] = Cell::Alive;
                        cells[((y + 2) * width + (x + 4)) as usize] = Cell::Alive;
                        cells[((y + 3) * width + (x + 0)) as usize] = Cell::Alive;
                        cells[((y + 3) * width + (x + 3)) as usize] = Cell::Alive;
                    },
                    2 => { // Diehard pattern (dies after 130 generations)
                        cells[((y + 0) * width + (x + 6)) as usize] = Cell::Alive;
                        cells[((y + 1) * width + (x + 0)) as usize] = Cell::Alive;
                        cells[((y + 1) * width + (x + 1)) as usize] = Cell::Alive;
                        cells[((y + 2) * width + (x + 1)) as usize] = Cell::Alive;
                        cells[((y + 2) * width + (x + 5)) as usize] = Cell::Alive;
                        cells[((y + 2) * width + (x + 6)) as usize] = Cell::Alive;
                        cells[((y + 2) * width + (x + 7)) as usize] = Cell::Alive;
                    },
                    3 => { // R-pentomino
                        cells[((y + 0) * width + (x + 1)) as usize] = Cell::Alive;
                        cells[((y + 0) * width + (x + 2)) as usize] = Cell::Alive;
                        cells[((y + 1) * width + (x + 0)) as usize] = Cell::Alive;
                        cells[((y + 1) * width + (x + 1)) as usize] = Cell::Alive;
                        cells[((y + 2) * width + (x + 1)) as usize] = Cell::Alive;
                    },
                    4 => { // Random block pattern
                        for i in 0..4 {
                            for j in 0..4 {
                                if Math::random() > 0.5 {
                                    cells[((y + i) * width + (x + j)) as usize] = Cell::Alive;
                                }
                            }
                        }
                    },
                    _ => { // Pulsar seed
                        for i in 0..3 {
                            for j in 0..3 {
                                cells[((y + i) * width + (x + j)) as usize] = Cell::Alive;
                            }
                        }
                    }
                }
            }
        }

        // Add significantly more random noise (25% of cells alive)
        for i in 0..width {
            for j in 0..height {
                if Math::random() < 0.25 {  // Increased from 0.1 to 0.25
                    cells[(j * width + i) as usize] = Cell::Alive;
                }
            }
        }

        Universe {
            width,
            height,
            cells,
            generation: 0,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        self.generation += 1;

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                // Add random chaos every 100 generations
                if self.generation % 100 == 0 && Math::random() < 0.01 {
                    next[idx] = if Math::random() < 0.5 { Cell::Alive } else { Cell::Dead };
                } else {
                    next[idx] = next_cell;
                }
            }
        }

        self.cells = next;
    }
}