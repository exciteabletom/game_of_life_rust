use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::vec::Vec;

type Grid = Vec<Vec<bool>>;

fn main() {
    let grid_data = read_file();
    let grid: Grid = string_to_vec(grid_data);
    println!("{:?}", grid);
}

fn read_file() -> String {
    let path = Path::new("grid.txt");

    let mut file = match File::open(&path) {
        Err(e) => panic!("Couldn't open file! {}", e),
        Ok(file) => file
    };

    let mut grid_data = String::new();
    match file.read_to_string(&mut grid_data) {
        Err(e) => panic!("Error reading file! {}", e),
        Ok(_) => {}
    }

    grid_data
}

fn string_to_vec(grid_data: String) -> Grid {
    let mut grid: Grid = vec![vec![]];
    for c in grid_data.chars() {
        let length = grid.len();
        if c == '\n' {
            grid.push(vec![]);
        } else if c == '0' {
            grid[length - 1].push(false);
        } else if c == '1' {
            grid[length - 1].push(true);
        } else {
            panic!("Unknown character '{}' in Grid file", c);
        }
    }

    let length = grid[0].len();
    for v in &grid {
        if v.len() != length {
            panic!("All rows must be of equal length!");
        }
    }
    grid
}
