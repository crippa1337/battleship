use std::{collections::HashMap, io::stdin};
fn main() {
    let mut player_arr = [[Square::Empty; 10]; 10];
    let mut comp_arr = [[Square::Empty; 10]; 10];
    print_board(&player_arr, false);
    println!("--------------------------------------\nINPUT IS TAKEN AS SUCH: [CHAR][INDEX]\nSUCH AS: a6, B7, j3\n--------------------------------------");

    loop {
        let input = take_cord_input(true);
        let cord = convert_input_to_array(input);
        player_arr[cord[1]][cord[0]] = Square::Ship;
        clear_screen();
        print_board(&player_arr, false);
    }
}

fn take_cord_input(ship_place: bool) -> String {
    if ship_place {
        println!("Place your ships");
    } else {
        println!("Take a shot");
    }
    loop {
        let mut input = String::new();

        loop {
            input.clear();
            stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();
            if input.len() == 2 {
                let mut tuple: (char, u8) = (' ', 0);
                for (i, c) in input.chars().enumerate() {
                    if i == 0 && c.is_ascii_alphabetic() {
                        tuple.0 = c;
                        return input;
                    } else if i == 1 && c.is_digit(10) {
                        tuple.1 = c as u8;
                        return input;
                    } else {
                        println!("'{}' is an invalid input", input);
                        break;
                    }
                }
            } else {
                println!("'{}' is an invalid input", input);
            }
        }
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
    let x = map.get(&x.to_string()).unwrap();
    let y = y.to_digit(10).unwrap() as usize;
    [*x, y]
}
