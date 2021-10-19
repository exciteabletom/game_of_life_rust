use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::vec::Vec;

type Grid = Vec<Vec<bool>>;

fn main() {
    let grid_data = read_file(); // Eventually this data will be entered through a UI
    let mut grid: Grid = string_to_vec(grid_data);

    pprint_grid(&grid);
    tick_grid(&mut grid);
    pprint_grid(&grid)
}

fn pprint_grid(grid: &Grid) {
    for row in grid.iter() {
        for cell in row.iter() {
            if *cell {
                print!("X");
            } else {
                print!("O");
            }
        }
        println!();
    }
    println!();
}

fn read_file() -> String {
    let path = Path::new("grid.txt");

    let mut file = match File::open(&path) {
        Err(e) => panic!("Couldn't open file! {}", e),
        Ok(file) => file,
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

fn tick_grid(grid: &mut Grid) {
    // This copy of the grid is readonly
    let grid_cpy = grid.clone();

    for (row_index, row) in grid_cpy.iter().enumerate() {
        for (index, live) in row.iter().enumerate() {
            let mut live_neighbours: u8 = 0;
            for dx in -1i64..1 {
                for dy in -1i64..1 {
                    // This check prevents "index out of bounds" panic
                    // TODO: Is there a cleaner way to do this without messy casts to i64?
                    if row_index as i64 + dy > (grid_cpy.len() as i64 - 1)
                        || index as i64 + dx > (row.len() as i64 - 1)
                        || row_index as i64 + dy < 0
                        || index as i64 + dx < 0
                    {
                        continue;
                    }
                    if grid_cpy[(row_index as i64 + dy) as usize][(index as i64 + dx) as usize] {
                        live_neighbours += 1
                    }
                }
            }
            if *live && ! (2..3).contains(&live_neighbours) {
                grid[row_index][index] = false
            } else if !*live && live_neighbours == 3 {
                grid[row_index][index] = true
            }
        }
    }
}
