use rand::{thread_rng, Rng};

pub fn generate(number: u32) -> Vec<String> {
    let mut coin_flip_streaks: Vec<String> = vec![];
    for _ in 0..number {
        let mut rng = thread_rng();
        match rng.gen_range(0..=1) {
            1 => coin_flip_streaks.push(String::from("T")),
            _ => coin_flip_streaks.push(String::from("H")),
        }
    }

    coin_flip_streaks
}

pub fn consecutive_probabilities() {
    let number = 100;
    let slice = 6;
    let amount = 10_000;
    let mut quantity = 0;

    let mut count_sub = 0;

    for _ in 0..amount {
        let result = generate(number);
        result.windows(slice).for_each(|sub| {
            let all_h = sub.iter().all(|chr| chr == "H");
            let all_t = sub.iter().all(|chr| chr == "T" );

            if all_h == true || all_t == true {
                quantity += 1;
                println!("{:?}", sub);
            }

            count_sub += 1;
        });
    }

    println!("Không gian lặp: {amount}");
    println!("Số lần thử: {number}");
    println!("Ngửa và lật liên tiếp: {slice}");
    println!("Kết quả: {:.2}", quantity as f32 / count_sub as f32);
}
