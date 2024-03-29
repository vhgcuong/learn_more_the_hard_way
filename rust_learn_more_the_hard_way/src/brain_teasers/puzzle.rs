use std::io::{self, stdin, Write};
use std::f32::consts::PI;

pub fn call_puzzle() {
    println!("Three and a Bit:{:<13}1", " ");
    println!("Non-standard Input:{:<10}2", " ");
    println!("Type Conversion:{:<13}3", " ");
    println!("Byte-Sized Chunks:{:<11}4", " ");
    println!("How Long Is a String?:{:<7}5", " ");
    println!("Please Reboot the Universe:{:<2}6", " ");
    println!("There and Back Again:{:<8}7", " ");

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
        3 => type_conversion(),
        4 => byte_sized(),
        5 => string_length(),
        6 => reboot_universe(),
        7 => there_and_back(),
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

///
/// Puzzle 3
///
///
pub fn type_conversion() {
    let x: u64 = 4_294_967_296;
    println!("x: {x}");
    let y = x as u32;
    println!("y: {y}");
    if x == y as u64 {
        println!("x equals y.");
    } else {
        println!("x does not equal y.");
    }

    let y32 = u32::MAX;
    println!("y32: {y32}");
    let y64: u64 = y32.into();
    println!("y32: {y32} => y64: {y64}");

    let z32: u32 = (5000_u64).try_into().expect("Conversion error");
    println!("5000 u64 => u32 {z32}");
}

///
/// Puzzle 4
///
pub fn byte_sized() {
    let mut counter: i8 = 0;
    loop {
        println!("{counter}");
        if counter.checked_add(1).is_some() {
            counter += 1;
        } else {
            break;
        }
    }
}

///
/// Puzzle 5
///
///

const HELLO_WORLD: &str = "Halló heimur";
pub fn string_length() {
    println!("{} is {} characters long.",
             HELLO_WORLD,
             HELLO_WORLD.len()
    );
    println!("{} is {} characters long.",
             HELLO_WORLD,
             HELLO_WORLD.chars().count()
    );
}

///
/// Puzzle 6
///
pub fn reboot_universe() {
    if 0.1 + 0.2 == 0.3 {
        println!("Arithmetic still works.");
    } else {
        println!("Please reboot the universe.")
    }
}




pub struct Degrees(pub f32);
pub struct Radians(pub f32);

impl Degrees {
    pub fn new(angle: f32) -> Self {
        Self(angle)
    }
}

impl From<Degrees> for Radians {
    fn from(item: Degrees) -> Self {
        Self(item.0 * PI / 180.0)
    }
}
pub fn there_and_back() {
    let one_eighty_degrees = Degrees::new(180.0);
    let one_eighty_radians: Radians = one_eighty_degrees.into();
    println!("180 Degrees in Radians = {}", one_eighty_radians.0);
}