use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::Duration;
use std::vec::Vec;
use std::thread::sleep;

type Grid = Vec<Vec<bool>>;

fn main() {
    let grid_data = read_file(); // Eventually this data will be entered through a UI
    let mut grid: Grid = string_to_vec(grid_data);

    loop {
        grid = tick_grid(&grid);
        pprint_grid(&grid);
        sleep(Duration::from_secs(1));
    }
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
        } else if c == 'O' {
            grid[length - 1].push(false);
        } else if c == 'X' {
            grid[length - 1].push(true);
        } else {
            panic!("Unknown character '{}' in Grid file", c);
        }
    }

    let length = grid[0].len();
    grid.truncate(grid.len() - 1);

    for v in &grid {
        if v.len() != length {
            println!("{:?}", v);
            panic!("All rows must be of equal length!");
        }
    }
    grid
}

fn tick_grid(grid: &Grid) -> Grid {
    let mut future: Grid = vec![vec![false; grid[0].len()]; grid.len()];

    for (row_index, row) in grid.iter().enumerate() {
        for (index, live) in row.iter().enumerate() {

            let mut live_neighbours: u8 = 0;

            for row_offset in [-1, 0, 1].iter() {
                for index_offset in [-1, 0, 1].iter() {

                    let offset_row = row_index as i64 + row_offset;
                    let offset_index = index as i64 + index_offset;

                    if offset_row < (grid.len() as i64)
                        && offset_index < (row.len() as i64)
                        && offset_row > 0
                        && offset_index > 0
                    {
                        if grid[offset_row as usize][offset_index as usize] == true {
                            live_neighbours += 1
                        }
                    }
                }
            }

            if *live {
                live_neighbours -= 1;
            }

            if *live && live_neighbours != 2 && live_neighbours != 3 {
                future[row_index][index] = false;
            } else if !*live && live_neighbours == 3 {
                future[row_index][index] = true;
            } else {
                future[row_index][index] = *live;
            }
        }
    }
    future
}
