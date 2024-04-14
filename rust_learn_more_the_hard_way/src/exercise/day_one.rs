use std::io;
use std::io::Write;

pub fn run_day_one() {
    println!("Loops:{:<19}1", " ");
    println!("Functions:{:<15}2", " ");
    println!("Macro:{:<19}3", " ");
    println!("Collatz Sequence:{:<8}4", " ");
    println!("Transpose:{:<15}5", " ");

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
        4 => println!("{}", collatz_length(11)),
        5 => println!("{:?}", transpose([[1, 2, 3], [4, 5, 6], [7, 8, 9]])),
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

pub fn collatz_length(mut n: i32) -> u32 {
    let mut len = 1;
    while n > 1 {
        n = if n % 2 == 0 {n/2} else {3*n+1};
        len += 1;
    }

    len
}

pub fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    println!("{:?}", matrix);
    unimplemented!()
}
