use std::fs::read_to_string;
use text_colorizer::*;

fn print_grid(grid: Vec<Vec<bool>>) {
    (0..grid.len()).for_each(|row| { 
        (0..grid[row].len()).for_each(|col| {
            if grid[row][col] { 
                print!("@"); 
            } else {
                print!(".");
            } 
        });
        println!();
    });
}

fn is_accessable(grid: &Vec<Vec<bool>>, i: usize, j: usize) -> bool {
    let mut paper_count = 0;
    
    if grid[i-1][j-1] {
        paper_count += 1;
    }

    if grid[i-1][j] {
        paper_count += 1;
    }

    if grid[i-1][j+1] {
        paper_count += 1;
    }

    if grid[i][j-1] {
        paper_count += 1;
    }

    if grid[i][j+1] {
        paper_count += 1;
    }

    if grid[i+1][j-1] {
        paper_count += 1;
    }

    if grid[i+1][j] {
        paper_count += 1;
    }

    if grid[i+1][j+1] {
        paper_count += 1;
    }

    paper_count < 4
}

fn part1(grid: Vec<Vec<bool>>) -> u64 {
    (1..grid.len()-1).fold(0, |acc, row| { 
        acc + (1..grid.len()-1).fold(0, |acc, col| {
            if grid[row][col] {
                if is_accessable(&grid, row, col) {
                    return acc + 1;
                }
            }
            acc
        })
    })
}

fn part2(mut grid: Vec<Vec<bool>>) -> u64 {
    let mut was_removed: bool = true;
    let mut sum: u64 = 0;
    
    while was_removed {
        was_removed = false;
        sum += (1..grid.len()-1).fold(0, |acc, row| { 
            acc + (1..grid.len()-1).fold(0, |acc, col| {
                if grid[row][col] {
                    if is_accessable(&grid, row, col) {
                        was_removed = true;
                        grid[row][col] = false;
                        return acc + 1;
                    }
                }
                acc
            })
        })
    }

    return sum;
}

fn main() {
    let data: Vec<Vec<bool>> = match read_to_string("inputs/data.txt") {
        Ok(v) => v.lines()
            .map(|x| { 
                x.chars()
                .map(|x| { if x == '@' {true} else {false}})
                .collect::<Vec<bool>>()
            })
            .collect::<Vec<Vec<bool>>>(),
        Err(e) => {
            eprintln!("{} failed to read from file: {:?}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    let data_size: usize = data.len() + 2;
    let mut grid = vec![vec![false; data_size]];

    (0..data.len()).for_each(|row| {
        grid.push(vec![false; data_size]);
        (0..data[row].len()).for_each(|col| {
            grid[row+1][col+1] = data[row][col]
        })
    });
    grid.push(vec![false; data_size]);

    println!("{} {}","Solution part one:".green().bold(), part1(grid.clone()));
    println!("{} {}","Solution part one:".green().bold(), part2(grid.clone()));
}
