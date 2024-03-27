use std::io::{self, stdin, Write};

pub fn call_puzzle() {
    println!("Three and a Bit:{:<9}1", " ");
    println!("Non-standard Input:{:<6}2", " ");

    println!("=====================================");
    print!("Lựa chọn puzzle: ");
    io::stdout().flush().expect("Không thể flush stdout");
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Khong the doc du lieu tu dong lenh");

    let input = match input.trim().parse::<u64>() {
        Ok(number) => number,
        Err(_) => {
            println!("Không thể chuyển đổi thành");
            return
        }
    };

    match input {
        1 => three_and_a_bit(),
        2 => standard_input(),
        _ => {}
    }
}

///
/// Puzzle 1
///
pub fn three_and_a_bit() {
    const THREE_AND_A_BIT: f32 = 3.4028236;
    println!("{}", THREE_AND_A_BIT);
}

///
/// Puzzle 2
/// Rust’s standard input system includes control sequences representing the
/// Enter key. \r indicates a carriage return, while \n indicates a line feed. You can
/// sanitize non-printing characters using the trim function.
///
pub fn standard_input() {
    println!("What is 3 + 2? Type your answer and press enter.");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Unable to read standarn input");
    println!("input: {:#?}", input);

    if input.trim() == "5" {    // if input == "5" {
        println!("Correct!");
    } else {
        println!("Incorrect!");
    }
}




