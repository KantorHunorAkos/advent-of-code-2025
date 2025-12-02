use std::fs::read_to_string;
use text_colorizer::*;

enum Direction {
    Left,
    Rigth,
}

fn direction_from_char(ch: char) -> Option<Direction> {
    match ch {
        'L' => Some(Direction::Left),
        'R' => Some(Direction::Rigth),
        _ => None
    }
}

fn part1(data: String) -> i64 {
    let mut position :i64 = 50;
    let mut sum :i64 = 0;
    for line in data.lines() {
        let mut chars = line.chars();
        let dir = match chars.next() {
            Some(v) => direction_from_char(v),
            None => {
                eprintln!("{} next() returned None", "Error:".red().bold());
                std::process::exit(1);
            }
        };
        
        let number = match chars.as_str().parse::<i64>() {
            Ok(v) => v,
            Err(e) => { 
                eprintln!("{} failed to parse number: {:?}", "Error:".red().bold(), e);
                std::process::exit(1);
            }
        };

        position = match dir {
            Some(v) => match v {
                Direction::Left => (position - number + 100) % 100,
                Direction::Rigth => (position + number) % 100
            }
            None => {
                eprintln!("{} failed to parse direction", "Error:".red().bold());
                std::process::exit(1);
            }
        };

        if position == 0 {
            sum += 1;
        }
    }

    sum
}

fn part2(data: String) -> i64 {
    let mut position : i64 = 50;
    let mut sum : i64 = 0;
    for line in data.lines() {
        let mut chars = line.chars();
        let dir = match chars.next() {
            Some(v) => direction_from_char(v),
            None => {
                eprintln!("{} next() returned None", "Error:".red().bold());
                std::process::exit(1);
            }
        };
        
        let mut number = match chars.as_str().parse::<i64>() {
            Ok(v) => v,
            Err(e) => { 
                eprintln!("{} failed to parse number: {:?}", "Error:".red().bold(), e);
                std::process::exit(1);
            }
        };

        sum += number / 100;
        number %= 100;

        position = match dir {
            Some(v) => match v {
                Direction::Left => {
                    if position != 0 && position - number < 0 {
                        println!("{}", position - number);
                        sum += 1;
                    }
                    (position - number + 100)%100
                },
                Direction::Rigth => position + number
            }
            None => {
                eprintln!("{} failed to parse direction", "Error:".red().bold());
                std::process::exit(1);
            }
        };

        if position == 0 {
            sum += 1;
        }
        
        sum += position / 100;
        position = position % 100;
    }

    sum
}

fn main() {
    let data = match read_to_string("inputs/data.txt") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to read from file: {:?}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };
    println!("{} {}", "Solution part one:".bold().green(), part1(data.clone()));
    println!("{} {}", "Solution part two:".bold().green(), part2(data.clone()));
}
