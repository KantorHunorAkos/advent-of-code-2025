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
    let mut data = match read_to_string("inputs/test_data.txt") {
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
        for col in 1..data[row].len() - 2 {
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

    let mut time: Vec<Vec<u64>> = vec![vec![0; data[0].len()]; data.len()];
    time[0][starting_point] = 1;

    for row in (2..time[0].len() - 1).step_by(2) {
        for col in 1..time[row].len() - 2 {
            if data[row][col] == Token::Beam {
                time[row][col] =
                    time[row - 2][col - 1] + time[row - 2][col] + time[row - 2][col + 1];
            }
        }
        println!("{:?}", time[row]);
    }

    println!(
        "{} {:?}",
        "Solution part two:".green().bold(),
        time[time.len() - 2].iter().sum::<u64>()
    );
}
