use std::io;

/* struct Tile { */
/*     number: u8, */
/* } */

/* fn print_grid() { */

/* } */

fn is_perfect_square(number: u8) -> bool {
    let sqrt = f32::sqrt(number.into()).floor();
    return sqrt * sqrt == number.into();
}

fn main() {
    let mut buffer = String::new();

    println!("Write the size of the grid (9x9 is default):");
    io::stdin().read_line(&mut buffer).expect("Failed to read from stdin");

    let trimmed = buffer.trim();
    match trimmed.parse::<u8>() {
        Ok(size) => {
            if is_perfect_square(size) {
                println!("Size is {}.", size);
            }
            else {
                println!("Grid needs to be a perfect square (4, 9, 16...)")f
            }
        },
        Err(..) => println!("Input is not an integer: {}.", trimmed),
    };
}
