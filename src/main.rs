use std::{collections::HashMap, io::stdin};
fn main() {
    let mut player_arr = [[Square::Empty; 10]; 10];
    let mut comp_arr = [[Square::Empty; 10]; 10];
    print_board(&player_arr);

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let new_cord = convert_input_to_array(input);
        player_arr[new_cord[1]][new_cord[0]] = Square::Ship;
        clear_screen();
        print_board(&player_arr);
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
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

fn convert_input_to_array(input: String) -> Vec<usize> {
    let map: HashMap<String, u8> = HashMap::from([
        (String::from("a"), 0),
        (String::from("b"), 1),
        (String::from("c"), 2),
        (String::from("d"), 3),
        (String::from("e"), 4),
        (String::from("f"), 5),
        (String::from("g"), 6),
        (String::from("h"), 7),
        (String::from("i"), 8),
        (String::from("j"), 9),
    ]);
    let buf = input.to_lowercase();
    let input = buf.trim();
    let input: Vec<&str> = input.split_whitespace().collect();
    let x = map.get(input[0]).unwrap();
    let x = *x as usize;
    let y = input[1].parse::<usize>().unwrap();
    let coordinate = vec![x, y];
    coordinate
}
