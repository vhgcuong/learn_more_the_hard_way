use std::io;

mod zigzag;
mod conway;

fn main() {
    println!("Zigzag: 1");
    println!("Conway: 2");


    let mut input = String::new();

    println!("Nhap mot so nguyen: ");
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
        2 => {
            let grid = conway::generate_grid(10, 10);
            conway::print_grid(grid.clone());
            let next_grid = conway::calculate(grid);
            conway::print_grid(next_grid);
        },
        _ => return
    }


}
