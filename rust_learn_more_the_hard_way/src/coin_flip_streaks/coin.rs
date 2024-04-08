use std::{io, io::Write};
use std::str::FromStr;
use rand::{thread_rng, Rng};

pub fn read_input<T: FromStr>(prompt: &str) -> Result<T, Box<dyn std::error::Error>> {
    print!("{}", prompt);
    io::stdout().flush().expect("Không thể flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Khong the doc du lieu tu dong lenh");

    println!("=====================================");
    input.trim().parse::<T>().map_err(|_| "Failed parsing input.".into())
}

pub struct CoinFlipSimulator {
    pub number_of_flips: u32,
    pub streak_length: usize,
    pub num_simulations: u32
}

impl CoinFlipSimulator {
    pub(crate) fn new() -> Self {
        let input: Result<u32, Box<dyn std::error::Error>> = read_input("Nhập số lần thử: ");
        let number_of_flips = input.unwrap_or_else(|e| 1);

        let input: Result<usize, Box<dyn std::error::Error>> = read_input("Nhập số lần ngửa và lật liên tiếp: ");
        let streak_length = input.unwrap_or_else(|e| 1);

        let input: Result<u32, Box<dyn std::error::Error>> = read_input("Nhập không gian lặp: ");
        let num_simulations = input.unwrap_or_else(|e| 1);

        CoinFlipSimulator {
            number_of_flips,
            streak_length,
            num_simulations
        }
    }
    fn generate(&self, number: u32) -> String {
        let mut coin_flip_streaks= String::new();
        for _ in 0..number {
            let mut rng = thread_rng();
            match rng.gen_range(0..=1) {
                1 => coin_flip_streaks += "T",
                _ => coin_flip_streaks += "H",
            }
        }

        coin_flip_streaks
    }
    pub(crate) fn probability_of_occurrence(&self) {
        let mut quantity = 0;
        let mut count_sub = 0;

        for _ in 0..self.num_simulations {
            let result = self.generate(self.number_of_flips);

            result
                .chars()
                .collect::<Vec<char>>()
                .windows(self.streak_length)
                .for_each(|sub| {
                    let all_h = sub.iter().all(|&chr| chr == 'H');
                    let all_t = sub.iter().all(|&chr| chr == 'T' );

                    if all_h || all_t {
                        quantity += 1;
                        println!("{:?}", sub);
                    }

                    count_sub += 1;
                });
        }

        println!("Không gian lặp: {}", self.num_simulations);
        println!("Số lần thử: {}", self.number_of_flips);
        println!("Ngửa và lật liên tiếp: {}", self.streak_length);
        println!("Kết quả: {:.2}", quantity as f32 / count_sub as f32);
    }
}

