extern crate ansi_term;

use ansi_term::Colour;

mod generators;
use generators::drunken_walk::DrunkenWalk;
use generators::monte_carlo::MonteCarlo;


fn main() {
    let width = 80;
    let height = 40;
    
    let mut board: [[usize; 80]; 40] = [[1; 80]; 40];

    let drunken_walk = DrunkenWalk{};
    board = drunken_walk.walk(width, height, board);

    // let cellular = Cellular{};
    // board = cellular.generate(width, height, board);

    println!();
    
    println!("{}", Colour::Green.paint("=== Cave Generation ==="));
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

    let monte_carlo = MonteCarlo{};
    let world = monte_carlo.generate_world();

    println!("{}", Colour::Green.paint("=== World Generation ==="));
    for row in world.iter() {
        for &tile in row.iter() {
            match tile {
                tile if tile <= -0.5 => print!("{}", Colour::Blue.paint("-")),
                tile if tile > -0.5 && tile <= 0.0 => print!("{}", Colour::Fixed(12).paint("~")),
                tile if tile > 0.0 && tile <= 0.5 => print!("{}", Colour::Green.paint(",")),
                tile if tile > 0.5 && tile < 0.75 => print!("#"),
                _ => print!("{}", Colour::Fixed(15).paint("*"))
            }
        }
        println!();
    }
    println!();

}
