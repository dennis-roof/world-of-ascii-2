use rand::prelude::*;

pub struct DrunkenWalk {}

impl DrunkenWalk {
    pub fn walk(&self, width: usize, height: usize, mut board: [[usize; 80]; 40]) -> [[usize; 80]; 40]
    {
        let remove_tiles = (width * height) / 2;
        let mut tiles_removed = 0;

        let mut x = rand::thread_rng().gen_range(0, width);
        let mut y = rand::thread_rng().gen_range(0, height);
        board[y][x] = 0;

        while tiles_removed < remove_tiles {
            match rand::thread_rng().gen_range(0, 4) {
                0 => if y > 0 { y -= 1; }, // up
                1 => if x < width-1 { x += 1; }, // right
                2 => if y < height-1 { y += 1; }, // down
                _ => if x > 0 { x -= 1; } // left
            }

            if board[y][x] == 1 {
                board[y][x] = 0;
                tiles_removed += 1;
            }
        }

        board
    }
}
