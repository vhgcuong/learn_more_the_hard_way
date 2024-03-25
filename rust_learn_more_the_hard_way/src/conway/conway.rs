use std::{io, thread, time};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use rand::{thread_rng, Rng};

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

pub fn calculate(grid: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let height = grid.len();
    let width = grid[0].len();
    // Contractor vector
    let mut next_cells = grid.clone();

    for x in 0..height {
        for y in 0..width {
            // Get neighboring coordinates:
            let left = ((x as i32 - 1) % width as i32) as usize ;
            let right = (x + 1) % width;
            let above = ((y as i32 - 1) % height as i32) as usize;
            let below = (y + 1) % height;

            // Count number of living neighbors:
            let mut num_neighbors = 0;
            if left < width
                && above < height
                && grid[left][above] == "#" {
                num_neighbors += 1;
            }
            if above < height
                && grid[x][above] == "#" {
                num_neighbors += 1;
            }
            if right < width
                && above < height
                && grid[right][above] == "#" {
                num_neighbors += 1;
            }
            if left < width
                && grid[left][y] == "#" {
                num_neighbors += 1;
            }
            if right < width
                && grid[right][y] == "#" {
                num_neighbors += 1;
            }
            if left < width
                && below < height
                && grid[left][below] == "#" {
                num_neighbors += 1;
            }
            if below < height
                && grid[x][below] == "#" {
                num_neighbors += 1;
            }
            if below < height
                && right < height
                && grid[right][below] == "#" {
                num_neighbors += 1;
            }

            // // Set cell based on Conway's Game of Life rules:
            match (grid[x][y].as_ref(), num_neighbors) {
                ("#", 2|3) => next_cells[x][y] = String::from("#"),
                ("_", 3) => next_cells[x][y] = String::from("#"),
                _ => next_cells[x][y] = String::from("_")
            }
        }
    }

    next_cells
}

pub fn input_data() -> Vec<i32> {
    println!("Width - Height: ");

    let mut data: Vec<i32> = vec![0, 0];
    for i in 0..data.len() {
        let mut input = String::new();

        print!("Nhap mot so nguyen: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Khong the doc du lieu tu dong lenh");

        let _ = match input.trim().parse::<i32>() {
            Ok(number) => data[i] = number,
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
    }
    println!("Got it! Exiting...");
}
