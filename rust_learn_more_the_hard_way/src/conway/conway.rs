use std::{thread, time};
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

pub fn print_grid(grid: Vec<Vec<String>>) {
    thread::sleep(time::Duration::from_millis(1_00));
    for row in grid {
        println!("{}", row.join(""));
        thread::sleep(time::Duration::from_millis(1_00));
    }
}

pub fn calculate(grid: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let height = grid.len();
    let width = grid[0].len();
    // Contractor vector
    let mut next_cells = grid.clone();

    for x in 0..width {
        for y in 0..height {
            // Get neighboring coordinates:
            let left = ((x as i32 - 1) % width as i32) as usize;
            let right = (x + 1) % width;
            let above = ((y as i32 - 1) % height as i32) as usize;
            let below = (y + 1) % height;

            // Count number of living neighbors:
            let mut num_neighbors = 0;
            println!("[{x} {y}] - [{left} {right} {above} {below}");
            if grid[left][above] == "#" {
                num_neighbors += 1;
            }
            if grid[x][above] == "#" {
                num_neighbors += 1;
            }
            if grid[right][above] == "#" {
                num_neighbors += 1;
            }
            if grid[left][y] == "#" {
                num_neighbors += 1;
            }
            if grid[right][y] == "#" {
                num_neighbors += 1;
            }
            if grid[left][below] == "#" {
                num_neighbors += 1;
            }
            if grid[x][below] == "#" {
                num_neighbors += 1;
            }
            if grid[right][below] == "#" {
                num_neighbors += 1;
            }

            // Set cell based on Conway's Game of Life rules:
            match (grid[x][y].as_ref(), num_neighbors) {
                ("#", 2|3) => next_cells[x][y] = String::from("#"),
                ("_", 3) => next_cells[x][y] = String::from("#"),
                _ => next_cells[x][y] = String::from("_")
            }
        }
    }

    next_cells
}
