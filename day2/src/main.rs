use std::fs::read_to_string;
use text_colorizer::*;

#[derive(Clone)]
struct Range {
    start: u64,
    end: u64,
}

fn convert_data(data: String) -> Vec<Range> {
    data.split(",")
        .map(|x| {
            let mut v = x.split("-").map(|x| match x.parse::<u64>() {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("{} failed to parse number: {:?}", "Error:".red().bold(), e);
                    std::process::exit(1);
                }
            });

            let mut range: Range = Range { start: 0, end: 0 };

            if let Some(start) = v.next() {
                range.start = start;
            } else {
                eprintln!("{} set start for range: {}", "Error:".red().bold(), x);
                std::process::exit(1);
            };

            if let Some(end) = v.next() {
                range.end = end;
            } else {
                eprintln!("{} set end for range: {}", "Error:".red().bold(), x);
                std::process::exit(1);
            };

            range
        })
        .collect::<Vec<Range>>()
}

fn is_part_one_invalid_id(mut n: u64) -> bool {
    let mut digits: Vec<u64> = vec![];
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }

    if digits.len() % 2 == 1 {
        return false;
    }

    for i in 0..digits.len() / 2 {
        if digits[i] != digits[digits.len() / 2 + i] {
            return false;
        }
    }
    true
}

fn is_part_two_invalid_id(mut n: u64) -> bool {
    let mut digits: Vec<u64> = vec![];
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }

    for window_size in 1..=digits.len() / 2 {
        if digits.len() % window_size != 0 {
            continue;
        }

        let mut repeting_window = true;
        let window: Vec<u64> = digits[0..window_size].to_vec();

        for i in 1..(digits.len() / window_size) {
            for j in 0..window_size {
                if window[j] != digits[j + i * window_size] {
                    repeting_window = false;
                }
            }
        }

        if repeting_window {
            return true;
        }
    }

    false
}

fn part1(data: Vec<Range>) -> u64 {
    data.iter().fold(0, |mut acc, x| {
        for n in x.start..=x.end {
            if is_part_one_invalid_id(n) {
                acc += n;
            }
        }
        acc
    })
}

fn part2(data: Vec<Range>) -> u64 {
    data.iter().fold(0, |mut acc, x| {
        for n in x.start..=x.end {
            if is_part_two_invalid_id(n) {
                acc += n;
            }
        }
        acc
    })
}

fn main() {
    let data = match read_to_string("inputs/data.txt") {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{} failed to read from file: {:?}",
                "Error:".red().bold(),
                e
            );
            std::process::exit(1);
        }
    };

    let data = convert_data(data);

    println!(
        "{} {}",
        "Part one solution:".bold().green(),
        part1(data.clone())
    );

    println!(
        "{} {}",
        "Part two solution:".bold().green(),
        part2(data.clone())
    );
}
