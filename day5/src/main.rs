use std::fs::read_to_string;
use text_colorizer::*;

#[derive(Clone, Ord, Eq, PartialEq, PartialOrd)]
struct Range {
    start: i64,
    end: i64
}

fn is_fresh(fresh_ids: &Vec<Range>, id: i64) -> bool {
    for i in 0..fresh_ids.len() {
        if id >= fresh_ids[i].start && id <= fresh_ids[i].end {
            return true;
        }
    }

    false
}

fn main() {
    let data = match read_to_string("inputs/test_data.txt") {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to read from file: {:?}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    let mut data = data.split("\n\n");

    let mut fresh = data.next().unwrap().lines()
        .map(|x| {
            let mut v = x.split("-").map(|x| match x.parse::<i64>() {
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
        .collect::<Vec<Range>>();

    println!("{} {}","Solution part one:".green().bold(), 
        data.next().unwrap()
            .lines()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
            .iter()
            .fold(0, |acc, x| if is_fresh(&fresh, *x) {acc +1} else {acc})
    );

    fresh.sort();
    let mut prev = Range{start: 0, end: 0};
    let mut sum_fresh_ids = 0;
    fresh.iter().for_each(|r| {
        sum_fresh_ids += if prev.end < r.start {r.end - r.start + 1} else {r.end - prev.start +1};
        prev = r.clone();
    });

    println!("{} {}","Solution part two:".green().bold(), sum_fresh_ids);
}
