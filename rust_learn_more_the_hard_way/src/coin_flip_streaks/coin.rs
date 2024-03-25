use rand::{thread_rng, Rng};

pub fn generate(number: u32) -> Vec<String> {
    let mut coin_flip_streaks: Vec<String> = vec![];
    for index in 0..number {
        let mut rng = thread_rng();
        match rng.gen_range(0..=1) {
            1 => coin_flip_streaks.push(String::from("T")),
            _ => coin_flip_streaks.push(String::from("H")),
        }
    }

    coin_flip_streaks
}

pub fn consecutive_probabilities(number_of_consecutive: u32) -> f32 {
    let mut probability: f32 = 0.0;


    probability
}


