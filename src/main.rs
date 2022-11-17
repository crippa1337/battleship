use std::io::stdin;
fn main() {
    let mut arr = [[Square::Empty; 10]; 10];
    print_board(&arr);

    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
}

#[derive(Clone, Copy)]
enum Square {
    Empty,
    Shot,
    Hit,
    Ship,
}

fn print_board(board: &[[Square; 10]; 10]) {
    use ansi_term::Color::{Cyan, Red, RGB};
    let mut row_id: i8 = -1;

    println!("      A   B   C   D   E   F   G   H   I   J");
    for (row_id, row) in board.iter().enumerate() {
        print!(
            "{}\n {}  ",
            RGB(100, 100, 100).paint("    +---+---+---+---+---+---+---+---+---+---+"),
            row_id
        );
        for &square in row {
            print!("{}", RGB(100, 100, 100).paint("| "));
            match square {
                Square::Empty => print!("  "),
                Square::Shot => print!("{}", Cyan.paint("O ")),
                Square::Hit => print!("{}", Red.bold().paint("X ")),
                Square::Ship => print!("# "),
            }
        }
        println!("{}", RGB(100, 100, 100).paint("|"));
    }
    println!(
        "{}",
        RGB(100, 100, 100).paint("    +---+---+---+---+---+---+---+---+---+---+")
    );
}
