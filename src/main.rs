use std::io::stdin;
fn main() {
    let arr = [[Square::Empty; 10]; 10];
    println!("{}", board_to_string(&arr));
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("Failed to read input");
}

#[derive(Clone, Copy)]
enum Square {
    Empty,
    Miss,
    Hit,
    Ship,
}

use std::fmt;
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Square::Empty => " ",
                Square::Miss => "O",
                Square::Hit => "X",
                Square::Ship => "#",
            }
        )
    }
}

// alternative crate https://crates.io/crates/tabled
fn board_to_string(board: &[[Square; 10]; 10]) -> String {
    use std::fmt::Write;
    let mut result = String::new();
    write!(result, "+ A + B + C + D + E + F + G + H + I + J +\n",).unwrap();
    for row in board {
        write!(result, "+---+---+---+---+---+---+---+---+---+---+\n",).unwrap();
        for square in row {
            write!(result, "| {} ", square).unwrap();
        }
        write!(result, "|\n").unwrap();
    }
    write!(result, "+---+---+---+---+---+---+---+---+---+---+",).unwrap();
    result
}
