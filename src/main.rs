mod generators;
use generators::drunken_walk::DrunkenWalk;
use generators::cellular::Cellular;

fn main() {
    let width = 80;
    let height = 40;
    
    let mut board: [[usize; 80]; 40] = [[1; 80]; 40];

    let drunken_walk = DrunkenWalk{};
    board = drunken_walk.walk(width, height, board);

    let cellular = Cellular{};
    board = cellular.generate(width, height, board);

    for row in board.iter() {
        for tile in row.iter() {
            match tile {
                1 => print!("#"),
                _ => print!(".")
            }
        }
        println!();
    }
    println!();
}
