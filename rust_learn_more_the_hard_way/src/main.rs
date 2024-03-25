use std::io;

mod zigzag;
mod conway;
mod coin_flip_streaks;

fn main() {
    println!("Zigzag:            1");
    println!("Conway:            2");
    println!("Coin Flip Streaks: 3");

    let mut input = String::new();

    print!("Nhap mot so nguyen: ");
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

    match input {
        1 => zigzag::zigzag(),
        2 => conway::game_of_life(),
        _ => return
    }


}
