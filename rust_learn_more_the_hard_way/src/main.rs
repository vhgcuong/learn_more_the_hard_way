use std::io;
use std::io::Write;
use crate::coin_flip_streaks::CoinFlipSimulator;

mod zigzag;
mod conway;
mod coin_flip_streaks;
mod brain_teasers;
mod exercise;
mod rusqlite_con;

fn main() {
    println!("Zigzag:{:<18}1", " ");
    println!("Conway:{:<18}2", " ");
    println!("Coin Flip Streaks:{:<7}3", " ");
    println!("Brain Teasers:{:<11}4", " ");
    println!("Exercise:{:<16}5", " ");

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
        3 => {
            let simulator = CoinFlipSimulator::new();
            simulator.probability_of_occurrence();
        },
        4 => brain_teasers::call_puzzle(),
        5 => exercise::run_day_one(),
        _ => ()
    }
}
