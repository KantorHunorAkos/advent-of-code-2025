use std::fs::read_to_string;
use std::str::FromStr;
use text_colorizer::*;

#[derive(Debug, PartialEq)]
enum Operator {
    Multiply,
    Add,
    None,
}

impl Operator {
    fn as_str(&self) -> &'static str {
        match self {
            Operator::Multiply => "*",
            Operator::Add => "+",
            Operator::None => "",
        }
    }
}

impl PartialEq<Operator> for &str {
    fn eq(&self, other: &Operator) -> bool {
        *self == other.as_str()
    }
}

struct Problem {
    op: Operator,
    operands: Vec<u64>,
    operands_part2: Vec<u64>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseOperatorError;

impl FromStr for Operator {
    type Err = ParseOperatorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == Operator::Add {
            return Ok(Operator::Add);
        }

        if s == Operator::Multiply {
            return Ok(Operator::Multiply);
        }

        if s == Operator::None {
            return Ok(Operator::None);
        }

        Err(ParseOperatorError)
    }
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

    let mut problems: Vec<Problem> = vec![];
    let mut first = true;
    data.lines().for_each(|x| {
        let line = x.split_whitespace().collect::<Vec<&str>>();

        if line[0] == Operator::Multiply || line[0] == Operator::Add {
            for i in 0..line.len() {
                problems[i].op = line[i].parse::<Operator>().unwrap();
            }
            return;
        }

        if first {
            first = false;

            for i in 0..line.len() {
                problems.push(Problem {
                    op: Operator::None,
                    operands: vec![],
                    operands_part2: vec![],
                });
                problems[i].operands.push(line[i].parse::<u64>().unwrap());
            }
            return;
        }

        for i in 0..line.len() {
            problems[i].operands.push(line[i].parse::<u64>().unwrap());
        }
    });

    println!(
        "{} {}",
        "Solution part one:".green().bold(),
        problems.iter().fold(0, |acc, p| {
            acc + match p.op {
                Operator::Add => p.operands.iter().fold(0, |acc, x| acc + x),
                Operator::Multiply => p.operands.iter().fold(1, |acc, x| acc * x),
                Operator::None => {
                    eprintln!("{} unknown operation", "Error:".red().bold(),);
                    std::process::exit(1);
                }
            }
        })
    );

    let operand_count = data.lines().count() - 1;
    let mut str_numbers: Vec<Vec<char>> = vec![];
    let mut i = 0;
    data.lines().for_each(|l| {
        if i == operand_count {
            return;
        }
        i += 1;
        let mut line = vec![];
        l.chars().into_iter().for_each(|c| {
            line.push(c);
        });
        str_numbers.push(line);
    });

    let mut index = 0;
    for col in 0..str_numbers[0].len() {
        let mut number: String = "".to_string();

        for row in 0..operand_count {
            number.push(str_numbers[row][col]);
        }

        let number = number.trim();
        if number.is_empty() {
            index += 1;
            continue;
        }

        problems[index].operands_part2.push(number.parse().unwrap());
    }

    println!(
        "{} {}",
        "Solution part two:".green().bold(),
        problems.iter().fold(0, |acc, p| {
            acc + match p.op {
                Operator::Add => p.operands_part2.iter().fold(0, |acc, x| acc + x),
                Operator::Multiply => p.operands_part2.iter().fold(1, |acc, x| acc * x),
                Operator::None => {
                    eprintln!("{} unknown operation", "Error:".red().bold(),);
                    std::process::exit(1);
                }
            }
        })
    );
}
