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
            if self.active_piece[i].1 < 0 {
                continue;
            }
            if !(self.active_piece[i].0+1 < self.pit_size.0 &&
               self.spots[self.active_piece[i].1 as usize][(self.active_piece[i].0+1) as usize] == false) {
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
            if self.active_piece[i].1 < 0 {
                continue;
            }
            if !(self.active_piece[i].0 > 0 &&
               self.spots[self.active_piece[i].1 as usize][(self.active_piece[i].0-1) as usize] == false) {
                return false
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
        println!("{:?}", completed_rows);
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
    let tetrominos = [
        [(3, 0), (4, 0), (5, 0), (6, 0)],  // I
        [(3, 0), (4, 0), (5, 0), (5, -1)],  // L
        [(3, -1), (3, 0), (4, 0), (5, 0)],  // J
        [(4, -1), (3, 0), (4, 0), (5, 0)],  // T
        [(4, 0), (5, 0), (5, -1), (6, -1)],  // S
        [(4, 0), (5, 0), (4, -1), (3, -1)],  // Z
        [(4, 0), (5, 0), (4, -1), (5, -1)],  // O
    ];
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0, 7);
    return tetrominos[6];
    tetrominos[index]
}


