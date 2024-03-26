use std::io;
use std::io::Write;

mod zigzag;
mod conway;
mod coin_flip_streaks;

fn main() {
    println!("Zigzag:{:<18}1", " ");
    println!("Conway:{:<18}2", " ");
    println!("Coin Flip Streaks:{:<7}3", " ");

    println!("=====================================");
    print!("Lựa chọn: ");
    io::stdout().flush().expect("Không thể flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Khong the doc du lieu tu dong lenh");

    let input = match input.trim().parse::<u64>() {
        Ok(number) => number,
        Err(_) => {
            println!("Không thể chuyển đổi thành");
            return
        }
    };
    println!("=====================================");

    match input {
        1 => zigzag::zigzag(),
        2 => conway::game_of_life(),
        3 => coin_flip_streaks::consecutive_probabilities(),
        _ => return
    }


}
