use std::time;
use rand::{thread_rng, Rng};

pub fn generate_grid(width: i32, height: i32) -> Vec<Vec<String>> {
    let mut next_cells = vec![];
    for x in (0..width) {
        let mut columns: Vec<String> = vec![];
        for y in (0..height) {
            let mut rng = thread_rng();
            match rng.gen_range(0..2) {
                1 => columns.push(String::from("#")),
                _ => columns.push(String::from(" ")),
            }
        }
        next_cells.push(columns)
    }

    next_cells
}


