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
        clear_screen();
        print_board(&player_arr, false);
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
    let input: Vec<char> = input.chars().collect();
    print!("{:?}", input);
    let mut new_cord = Vec::new();
    new_cord.push(map[&input[0].to_string()] as usize);
    new_cord.push(input[1].to_digit(10).unwrap() as usize);
    new_cord
}
