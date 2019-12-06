pub struct Cellular {}

impl Cellular {
    pub fn generate(&self, width: usize, height: usize, mut board: [[usize; 80]; 40]) -> [[usize; 80]; 40]
    {
        let max_generations = 3;
        let mut current_generation = 0;
        let mut x;
        let mut y;
        let mut wall_count;

        let mut next_board: [[usize; 80]; 40] = [[1; 80]; 40];

        while current_generation < max_generations {
            y = 1;
            while y < height-2 {
                x = 1;
                while x < width-2 {
                    next_board[y][x] = 0;
                    
                    if board[y][x] == 0 {
                        wall_count = 0;
                        wall_count += board[y-1][x-1];
                        wall_count += board[y-1][x];
                        wall_count += board[y-1][x+1];
                        wall_count += board[y][x-1];
                        wall_count += board[y][x+1];
                        wall_count += board[y+1][x-1];
                        wall_count += board[y][x];
                        wall_count += board[y][x+1];

                        if wall_count >= 5 {
                            println!("wall up! {},{}", y, x);
                            next_board[y][x] = 1;
                        }
                    }

                    x += 1;
                }
                
                y += 1;
            }

            println!("=== Generation {} ===", current_generation);
            for row in board.iter() {
                for tile in row.iter() {
                    match tile {
                        1 => print!("#"),
                        _ => print!(".")
                    }
                }
                println!();
            }
            println!("===");
            for row in next_board.iter() {
                for tile in row.iter() {
                    match tile {
                        1 => print!("#"),
                        _ => print!(".")
                    }
                }
                println!();
            }
            println!("===");

            board = next_board.clone();
            current_generation += 1;
        }

        board
    }
}
