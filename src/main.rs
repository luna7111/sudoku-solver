use std::io;
use std::process;

#[derive(Clone)]
struct Tile {
    is_set: bool,
    in_conflict: bool,
    number: char,
}

struct Cursor {
    x: usize,
    y: usize,
}

struct Board <'a>{
    cursor: Cursor,
    size: usize,
    base: &'a str,
    region_size: usize,
    grid: Vec<Vec<Tile>>,
}

fn is_valid_size(number: usize) -> bool {
    match number {
        4 | 9 | 16 => true,
        _ => false,
    }
}

const COLOR_RESET: &str = "\u{1b}[0m";
const COLOR_F_BLUE: &str = "\u{1b}[34m";
const COLOR_F_GREEN: &str = "\u{1b}[33m";
const COLOR_B_RED: &str = "\u{1b}[41m";
const COLOR_B_MAGENTA: &str = "\u{1b}[45m";

fn evaluate_constrains(board: &mut Board) {
    // for row in board.grid.iter_mut() {
    //     for mut tile in &mut *row {
    //         let mut ocurrences = 0;
    //         for check_tile in row.clone() {
    //             if check_tile.is_set && check_tile.number == tile.number {
    //                 ocurrences += 1;
    //             }
    //         }
    //         match ocurrences {
    //             1 => tile.in_conflict = true,
    //             _ => tile.in_conflict = false,
    //         }

    //     }
    // }
}

fn print_grid(board: &mut Board) {

    evaluate_constrains(board);

    for (row_index, row) in board.grid.iter().enumerate() {
        for (tile_index, tile) in row.iter().enumerate() {
            let color = match (row_index / board.region_size + tile_index / board.region_size) % 2 {
                0 => COLOR_F_BLUE,
                1 => COLOR_F_GREEN,
                _ => {
                    println!("Mathematics broke.");
                    COLOR_F_GREEN
                },
            }; 

            let bg_color = 
            if board.grid[board.cursor.y][board.cursor.x].in_conflict == true {
                COLOR_B_RED
            }
            else if board.cursor.y == row_index && tile_index == board.cursor.x {
                COLOR_B_MAGENTA
            } 
            else {
                ""
            };

            match tile.is_set{
                true => print!("{}{}{}{} ", color, bg_color,  tile.number, COLOR_RESET),
                false => print!("{}{}▢ {}", color, bg_color, COLOR_RESET),
            }
        }
        println!("");
    }
}

fn get_insert_input(board: &mut Board) {
    let mut input_buffer = String::new();

    println!("Insert a number:");
    io::stdin().read_line(&mut input_buffer).expect("Failed to read from stdin");

    let trimmed = input_buffer.trim();
    if board.base.contains(trimmed) {
        board.grid[board.cursor.y][board.cursor.x].is_set = true;
        board.grid[board.cursor.y][board.cursor.x].number = trimmed.chars().next().expect("Not a valid number.");
    }
    else {
        println!("Not a valid number.");
    }

}

fn get_input(board: &mut Board) {
    let mut input_buffer = String::new();

    println!("WASD to move cursor, I to insert a value in the tile U to unset value, Q to quit.");
    io::stdin().read_line(&mut input_buffer).expect("Failed to read from stdin");

    let trimmed = input_buffer.trim();
    match trimmed {
    
        "w" => {
            if board.cursor.y > 0 {
                board.cursor.y -= 1;
            }
        },
        "s" => {
            if board.cursor.y < board.size - 1 {
                board.cursor.y += 1;
            }
        },
        "a" => {
            if board.cursor.x > 0 {
                board.cursor.x -= 1;
            }
        },
        "d" => {
            if board.cursor.x < board.size - 1 {
                board.cursor.x += 1;
            }
        },
        "i" => {
            get_insert_input(board);
        },
        "u" => {
            board.grid[board.cursor.y][board.cursor.x].is_set = false;
        }
        "q" => {
            println!("quit");
            process::exit(0);
        }
        _ => println!("Bad input :("),
    }
}

const BASE_4x4: &str = "1234";
const BASE_9x9: &str = "123456789";
const BASE_16x16: &str = "0123456789abcdef";

fn start_grid(size: usize) {

    let mut board = Board {
        cursor: Cursor {x: 0, y: 0},
        size: size,
        base: match size {
            4 => BASE_4x4,
            9 => BASE_9x9,
            16 => BASE_16x16,
            _ => BASE_9x9,
        },
        region_size: f32::sqrt(size as f32).floor() as usize,
        grid: vec![vec![Tile { is_set: false, in_conflict: false, number: '0' }; size]; size],
    };

    loop {
        print_grid(&mut board);
        get_input(&mut board);
    }
}

fn main() {
    let mut input_buffer = String::new();

    println!("Write the size of the grid (4, 9, 16):");
    io::stdin().read_line(&mut input_buffer).expect("Failed to read from stdin");

    let trimmed = input_buffer.trim();
    match trimmed.parse::<usize>() {
        Ok(size) => {
            if is_valid_size(size) {
                println!("Size is {}.", size);
                start_grid(size);
            }
            else {
                println!("Invalid grid size (4, 9, 16)");
            }
        },
        Err(..) => println!("Input is not an integer: {}.", trimmed),
    };
}
