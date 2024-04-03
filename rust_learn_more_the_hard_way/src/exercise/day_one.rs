use std::io;
use std::io::Write;

pub fn run_day_one() {
    println!("Loops:{:<19}1", " ");
    println!("Functions:{:<15}2", " ");
    println!("Macro:{:<19}3", " ");

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
        1 => labels(),
        2 => println!("{}", gcd(100, 40)),
        3 => println!("{}", factorial(10)),
        _ => ()
    }
}

pub fn labels() {
    let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    println!("{:#?}", s);

    let mut elements_searched = 0;
    let target_value = 10;
    'outer: for &row in &s {
        for item in row {
            elements_searched += 1;
            if item == target_value {
                break 'outer;
            }
        }
    }
    print!("elements searched: {elements_searched}");
}

pub fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 {
        gcd(b, a % b)
    } else {
        a
    }
}

pub fn factorial(n: u32) -> u32 {
    let mut product = 1;
    for i in 1..=n {
        product *= dbg!(i);
        dbg!(product);
    }
    product
}

pub fn fizzbuzz(n: u32) -> u32 {
    todo!()
}