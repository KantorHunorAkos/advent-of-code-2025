use std::fs::read_to_string;
use text_colorizer::*;

fn part1(data: Vec<Vec<u32>>) -> u32 {
    data.iter().fold(0, |acc, x| { 
        let m = x.iter().max().unwrap(); 
        let p = x.iter().position(|x| x==m).unwrap();
    
        if p == x.len()-1 {
            let first = x[0..x.len()-1].iter().max().unwrap();
            return acc + first*10 + m;
        }

        let second = x[p+1..x.len()].iter().max().unwrap();
        acc + m*10 + second
    })
}

fn get_joltage_for_bank(bank: Vec<u32>, n: usize) -> u64 {
    let mut number: u64 = 0;
    let mut pos: usize = 0;

    (0..n).rev().for_each(|i| {
        if let Some(value) = bank[pos..bank.len()-i].iter().max() {
            number = number * 10 + (*value as u64);
            pos += bank[pos..bank.len()-i].iter().position(|x| x == value).unwrap() + 1;
        } else {
            eprintln!("{} maximum value not found", "Error:".red().bold());
            std::process::exit(1);
        }
    });

    number
}

fn get_total_joltage(banks: Vec<Vec<u32>>, battery_count: usize) -> u64 {
    banks.iter()
        .map(|bank| get_joltage_for_bank(bank.to_vec(), battery_count))
        .sum()

}


fn main() {
    let data = match read_to_string("inputs/data.txt") {
        Ok(v) => v.lines()
            .map(|x| 
                x.chars()
                .map(|x| 
                    x
                    .to_digit(10)
                    .unwrap()
                    )
                .collect::<Vec<u32>>()
                )
            .collect::<Vec<Vec<u32>>>(),
        Err(e) => {
            eprintln!("{} failed to read from file: {:?}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    println!("{} {}","Solution part one:".green().bold(), part1(data.clone()));
    println!("{} {}","Solution part two:".green().bold(), get_total_joltage(data.clone(), 12));
}
