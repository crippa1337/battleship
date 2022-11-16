use std::io::stdin;

fn take_input(input: &str) -> String {
    println!("{input}");
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .expect("Failed to read buffer in input");
    buf = buf.trim().to_lowercase();
    return buf;
}
fn main() {
    let board_size = take_input("what is your amongus");
    board_size = board_size.parse::<u8>();
    println!("{boarz_size}")
}
