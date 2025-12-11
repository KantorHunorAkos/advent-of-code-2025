use std::fs::read_to_string;
use text_colorizer::*;

#[derive(Debug, PartialEq)]
enum Token {
    Empty,
    Start,
    Beam,
    Splitter,
}

impl Token {
    fn from(ch: char) -> Option<Token> {
        match ch {
            '.' => Some(Token::Empty),
            'S' => Some(Token::Start),
            '|' => Some(Token::Beam),
            '^' => Some(Token::Splitter),
            _ => None,
        }
    }

    fn as_char(&self) -> char {
        match self {
            Token::Empty => '.',
            Token::Start => 'S',
            Token::Beam => '|',
            Token::Splitter => '^',
        }
    }
}

impl PartialEq<Token> for char {
    fn eq(&self, other: &Token) -> bool {
        *self == other.as_char()
    }
}

fn main() {
    let mut data = match read_to_string("inputs/data.txt") {
        Ok(v) => v
            .lines()
            .map(|l| {
                let mut line = vec![{ Token::Empty }];
                for c in l.chars() {
                    if let Some(token) = Token::from(c) {
                        line.push(token);
                    } else {
                        eprintln!("{} unknown token: {:?}", "Error:".red().bold(), c);
                        std::process::exit(1);
                    }
                }
                line.push(Token::Empty);
                line
            })
            .collect::<Vec<Vec<Token>>>(),
        Err(e) => {
            eprintln!(
                "{} failed to read from file: {:?}",
                "Error:".red().bold(),
                e
            );
            std::process::exit(1);
        }
    };

    let starting_point = match data[0].iter().position(|token| *token == Token::Start) {
        Some(v) => v,
        None => {
            eprintln!("{} failed find starting point", "Error:".red().bold(),);
            std::process::exit(1);
        }
    };

    data[1][starting_point] = Token::Beam;

    let mut split_count = 0;
    for row in 2..data[0].len() - 1 {
        for col in 1..data[row].len() - 1 {
            if data[row][col] == Token::Splitter && data[row - 1][col] == Token::Beam {
                split_count += 1;
                data[row][col - 1] = Token::Beam;
                data[row][col + 1] = Token::Beam;
            }

            if data[row][col] == Token::Empty && data[row - 1][col] == Token::Beam {
                data[row][col] = Token::Beam;
            }
        }
    }

    println!("{} {}", "Solution part one:".green().bold(), split_count);

    let mut time: Vec<u64> = vec![0; data[0].len()];
    time[starting_point] = 1;

    for row in (0..data.len()).step_by(2) {
        for col in 1..data[row].len() - 1 {
            if data[row][col] == Token::Splitter {
                time[col + 1] += time[col];
                time[col - 1] += time[col];
                time[col] = 0;
            }
        }
    }

    println!(
        "{} {:?}",
        "Solution part two:".green().bold(),
        time.iter().sum::<u64>()
    );
}
