type Cell = u8;

pub struct Grid {
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn possible(&self, row: Cell, col: Cell, n: Cell) -> bool {
        for i in 0..9 {
            if self.cells[(row * 9 + i) as usize] == n
                || self.cells[(i * 9 + col) as usize] == n
                || self.cells[((row / 3 * 3 + i / 3) * 9 + col / 3 * 3 + i % 3) as usize] == n
            {
                return false;
            }
        }
        true
    }

    pub fn solve(&mut self) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if self.cells[(row * 9 + col) as usize] == 0 {
                    for n in 1..=9 {
                        if self.possible(row, col, n) {
                            self.cells[(row * 9 + col) as usize] = n;
                            if self.solve() {
                                return true;
                            }
                            self.cells[(row * 9 + col) as usize] = 0;
                        }
                    }
                    return false;
                }
            }
        }
        true
    }

    pub fn new(start: &Option<Vec<Option<Cell>>>) -> Grid {
        let mut grid = Grid { cells: vec![0; 81] };

        if let Some(start) = start {
            for (idx, i) in start.iter().enumerate() {
                if let Some(n) = *i {
                    if !(1..=9).contains(&n) {
                        panic!("Invalid number: {}", n);
                    }
                    grid.cells[idx] = n;
                }
            }
        }

        grid
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, cell) in self.cells.iter().enumerate() {
            if i % 9 == 0 {
                writeln!(f)?;
            }
            write!(f, "{} ", cell)?;
        }
        writeln!(f)
    }
}
