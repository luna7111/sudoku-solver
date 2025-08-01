use std::io;
use color_print::cprint;

#[derive(Clone)]
struct Tile {
    is_set: bool,
    number: usize,
}

struct Board {
    size: usize,
    region_size: usize,
    grid: Vec<Vec<Tile>>,
}

fn is_perfect_square(number: usize) -> bool {
    let sqrt = f32::sqrt(number as f32).floor();
    return sqrt * sqrt == number as f32;
}

fn print_grid(board: Board) {
    for (row_index, row) in board.grid.iter().enumerate() {
        for (tile_index, tile) in row.iter().enumerate() {
            let color = if (row_index / board.region_size + tile_index / board.region_size) % 2 == 0 {"\u{1b}[34m"} else {"\u{1b}[33m"};
            if tile.is_set == true {
                cprint!("{}{}", color,  tile.number);
            }
            else {
                cprint!("{}#", color);
            }
        }
        println!("");
    }
}

fn start_grid(size: usize) {

    let mut board = Board {
        size: size,
        region_size: f32::sqrt(size as f32).floor() as usize,
        grid: vec![vec![Tile { is_set: false, number: 0 }; size]; size],
    };

   print_grid(board);
}

fn main() {
    let mut buffer = String::new();

    println!("Write the size of the grid (9x9 is default):");
    io::stdin().read_line(&mut buffer).expect("Failed to read from stdin");

    let trimmed = buffer.trim();
    match trimmed.parse::<usize>() {
        Ok(size) => {
            if is_perfect_square(size) {
                println!("Size is {}.", size);
                start_grid(size);
            }
            else {
                println!("Grid needs to be a perfect square (4, 9, 16...)");
            }
        },
        Err(..) => println!("Input is not an integer: {}.", trimmed),
    };
}
