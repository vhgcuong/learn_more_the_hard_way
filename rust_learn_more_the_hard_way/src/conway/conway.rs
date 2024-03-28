use std::{
    io::{self, Write},
    thread,
    time
};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use rand::{thread_rng, Rng};

static NEIGHBOURHOOD_OFFSETS: &[(i32, i32)] = &[
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

pub fn generate_grid(width: i32, height: i32) -> Vec<Vec<String>> {
    let mut next_cells = vec![];
    for _ in 0..height {
        let mut columns: Vec<String> = vec![];
        for _ in 0..width {
            let mut rng = thread_rng();
            match rng.gen_range(0..2) {
                1 => columns.push(String::from("#")),
                _ => columns.push(String::from("_")),
            }
        }
        next_cells.push(columns)
    }

    next_cells
}

pub fn print_grid(grid: &Vec<Vec<String>>) {
    thread::sleep(time::Duration::from_millis(1_00));
    for row in grid {
        println!("{}", row.join(""));
        thread::sleep(time::Duration::from_millis(1_00));
    }
    println!();
}

pub fn calculate(grid: &[Vec<String>]) -> Vec<Vec<String>> {
    let height = grid.len();
    let width = grid[0].len();
    // Contractor vector
    let mut next_cells = grid.to_owned();

    for x in 0..height {
        for y in 0..width {
            let mut num_neighbors = 0;
            // Get neighboring coordinates:
            NEIGHBOURHOOD_OFFSETS.iter().for_each(|&(ox, oy)| {
                let nx = x as i32 + ox;
                let ny = y as i32 + oy;
                if nx >= 0
                    && ny >= 0
                    && nx < height as i32
                    && ny < width as i32
                    && grid[nx as usize][ny as usize] == "#" {
                    num_neighbors += 1;
                }
            });

            // Set cell based on Conway's Game of Life rules:
            next_cells[x][y] = match (grid[x][y].as_ref(), num_neighbors) {
                ("#", 2|3) => String::from("#"),
                ("_", 3) => String::from("#"),
                _ => String::from("_")
            }
        }
    }

    next_cells
}

pub fn input_data() -> Vec<i32> {
    let mut data: Vec<i32> = vec![0, 0];

    for (i, item) in data.iter_mut().enumerate() {
        print!("{}: ", if i == 0 {"Width"} else {"Height"});
        io::stdout().flush().expect("Không thể flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Khong the doc du lieu tu dong lenh");

        match input.trim().parse::<i32>() {
            Ok(number) => *item = number,
            Err(_) => {
                println!("Không thể chuyển đổi thành");
                break;
            }
        };
    }

    data
}

pub fn game_of_life() {
    let data = input_data();

    let base_grid = generate_grid(data[0], data[1]);
    print_grid(&base_grid);

    let mut grid = base_grid.clone();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    println!("Waiting for Ctrl-C...");
    while running.load(Ordering::SeqCst) {
        grid = calculate(&grid);
        print_grid(&grid);

        // Dừng khi tất cả đều chết
        let pause = grid.iter().all(|row| {
            row.iter().all(|cell| {
                cell == "_"
            })
        });

        if pause {
            break;
        }
    }
    println!("Got it! Exiting...");
}
