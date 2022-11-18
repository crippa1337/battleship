use std::{collections::HashMap, io::stdin};
fn main() {
    let mut player_arr = [[Square::Empty; 10]; 10];
    let mut comp_arr = [[Square::Empty; 10]; 10];
    print_board(&player_arr, false);

    loop {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let cord = convert_input_to_array(input);
        player_arr[cord[1]][cord[0]] = Square::Ship;
        print_board(&player_arr, false);
        clear_screen();
    }
}

fn ship_placement() {}

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

fn print_board(board: &[[Square; 10]; 10], comp_board: bool) {
    use ansi_term::Color::{Cyan, Red, RGB};

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
                Square::Ship => {
                    if comp_board {
                        print!("  ")
                    } else {
                        print!("# ")
                    }
                }
            }
        }
        println!("{}", RGB(100, 100, 100).paint("|"));
    }
    println!(
        "{}",
        RGB(100, 100, 100).paint("    +---+---+---+---+---+---+---+---+---+---+")
    );
}

fn convert_input_to_array(input: String) -> [usize; 2] {
    let map: HashMap<String, usize> = HashMap::from([
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
    let mut input = input.chars();
    let x = input.next().unwrap();
    let y = input.next().unwrap();
    let x = map.get(&x.to_string());
    let x = match x {
        Some(x) => *x,
        None => 10,
    };
    let y = y.to_digit(10);
    let y = match y {
        Some(y) => y as usize,
        None => 10,
    };
    match y {
        0..=9 => y,
        _ => 10,
    };
    [x, y]
}
