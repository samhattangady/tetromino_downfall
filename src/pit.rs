extern crate rand;
use rand::Rng;

#[derive(Debug)]
pub struct Pit {
    pit_size: (i32, i32),
    pub active_piece: [(i32, i32); 4],
    spots: Vec<Vec<bool>>,
}

impl Pit {
    pub fn new(width: i32, height: i32) -> Pit {
        let mut spots = Vec::new();
        for _y in 0..height {
            let mut row = Vec::new();
            for _x in 0..width {
                row.push(false);
            }
            spots.push(row);
        }
        Pit {
            pit_size: (width, height),
            active_piece: get_random_tetromino(),
            spots: spots,
        }
    }

    pub fn move_piece_right(&mut self) {
        if self.can_move_right() {
            self.move_right();
        }
    }
    fn can_move_right(&self) -> bool {
        for i in 0..4 {
            if self.active_piece[i].0+1 >= self.pit_size.0 {
                return false;
            }
            if self.active_piece[i].1 < 0 {
                continue;
            }
            if self.spots[self.active_piece[i].1 as usize][(self.active_piece[i].0+1) as usize] == true {
                return false
            }
        }
        true
    }
    fn move_right(&mut self) {
        for i in 0..4 {
            self.active_piece[i].0 += 1
        }
    }

    pub fn move_piece_left(&mut self) {
        if self.can_move_left() {
            self.move_left();
        }
    }
    fn can_move_left(&self) -> bool {
        for i in 0..4 {
            if self.active_piece[i].0 <= 0 {
                return false;
            }
            if self.active_piece[i].1 < 0 {
                continue;
            }
            if self.spots[self.active_piece[i].1 as usize][(self.active_piece[i].0-1) as usize] == true {
                return false;
            }
        }
        true
    }
    fn move_left(&mut self) {
        for i in 0..4 {
            self.active_piece[i].0 -= 1
        }
    }

    pub fn move_piece_down(&mut self) {
        if self.can_move_down() {
            self.move_down();
        } else {
            self.solidify_piece();
            self.clear_completed_lines();
            self.generate_new_piece();
        }
    }
    fn can_move_down(&self) -> bool {
        for i in 0..4 {
            if !(self.active_piece[i].1+1 < self.pit_size.1 &&
               self.spots[(self.active_piece[i].1+1) as usize][self.active_piece[i].0 as usize] == false) {
                return false
            }
        }
        true
    }
    fn move_down(&mut self) {
        for i in 0..4 {
            self.active_piece[i].1 += 1
        }
    }

    pub fn rotate_piece(&mut self) {
        // FIXME (26 Aug 2019 sam): Buggy. Pieces can rotate out of the playing
        // field, or onto existing blocks. Need to add error checks here.
        self.active_piece = rotate_tetromino(self.active_piece); 
    }

    fn solidify_piece(&mut self) {
        for i in 0..4 {
            self.spots[self.active_piece[i].1 as usize][self.active_piece[i].0 as usize] = true
        }
    }

    fn clear_completed_lines(&mut self) {
        let mut completed_rows = std::vec::Vec::new();
        for (index, row) in self.spots.iter().enumerate() {
            let mut complete = true;
            for block in row {
                if !block {
                    complete = false;
                    break
                }
            }
            if complete {
                completed_rows.push(index);
            }
        }
        for index in completed_rows.iter().rev() {
            self.spots.remove(*index);
        }
        for _ in completed_rows {
            self.spots.insert(0, vec![false; self.pit_size.0 as usize]);
        }
    }

    fn generate_new_piece(&mut self) {
        // TODO (Aug 26 2019 sam): Make sure that the tetromino spots are not already occupued?
        // Lose condition?
        self.active_piece = get_random_tetromino();
    }

    pub fn get_solid_blocks(&self) -> Vec<(i32, i32)> {
        let mut blocks = std::vec::Vec::new();
        for x in 0..self.pit_size.0 {
            for y in 0..self.pit_size.1 {
                if self.spots[y as usize][x as usize] {
                    blocks.push((x, y));
                } 
            }
        }
        blocks
    }
}

fn get_random_tetromino() -> [(i32, i32); 4] {
    // We store the 0th block as the one that we want to rotate about.
    // FIXME (26 Aug 2019 sam): This will allow o piece to rotate...
    let tetrominos = [
        [(4, 0), (3, 0), (5, 0), (6, 0)],  // I
        [(4, 0), (3, 0), (5, 0), (5, -1)],  // L
        [(3, 0), (3, -1), (4, 0), (5, 0)],  // J
        [(4, 0), (3, 0), (4, -1), (5, 0)],  // T
        [(5, 0), (4, 0), (5, -1), (6, -1)],  // S
        [(5, 0), (4, 0), (4, -1), (3, -1)],  // Z
        [(4, 0), (5, 0), (4, -1), (5, -1)],  // O
    ];
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0, 7);
    tetrominos[index]
}

fn rotate_tetromino(piece: [(i32, i32); 4]) -> [(i32, i32); 4] {
    // Figured out the logic by working out on paper
    // We set the 0th block as "origin". Then get the relative pos of each
    // block based on that. For clockwise rotation, the new position of each
    // block has: (-y, x)
    // FIXME (26 Aug 2019 sam): Piece rotation doesn't feel so good. The S and
    // z pieces lose their vertical paths on rotation. Doesn't feel right.
    let origin = piece[0];
    let mut rotated = [(0, 0); 4];
    for i in 0..4 {
        // translated[i] would be (piece[i].0-origin.0, piece[i].1-origin.1)
        rotated[i] = (-(piece[i].1-origin.1), piece[i].0-origin.0);
        rotated[i] = (rotated[i].0+origin.0, rotated[i].1+origin.1);
    }
    rotated
}
