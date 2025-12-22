use std::fs::read_to_string;
use text_colorizer::*;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Point{
    x: i64,
    y: i64,
    z: i64
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Path {
    from: Point,
    to: Point,
    length: i64
}

impl Point {
    fn distance(&self, other: &Point) -> i64 {
            (self.x-other.x).pow(2) +
            (self.y-other.y).pow(2) +
            (self.z-other.z).pow(2)
    }
}

fn main() {
    let pair_count = 1000;
    let data = match read_to_string("inputs/data.txt") {
        Ok(v) => v
            .lines()
            .map(|l| {
                let point_vec = l
                    .split(",")
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>();
                Point{
                    x: point_vec[0],
                    y: point_vec[1],
                    z: point_vec[2],
                }
            })
            .collect::<Vec<Point>>(),
        Err(e) => {
            eprintln!(
                "{} failed to read from file: {:?}",
                "Error:".red().bold(),
                e
            );
            std::process::exit(1);
        }
    };

    let mut paths = vec![];

    for i in 0..data.len()-1 {
        for j in i+1..data.len() {
            paths.push(Path{
                from: data[i],
                to: data[j],
                length: data[i].distance(&data[j])
            });
        }
    }

    paths.sort_by(|a, b| a.length.cmp(&b.length));
    
    let mut circuits: Vec<Vec<Point>> = vec![];
    for path in paths[0..pair_count].iter() {
        let mut from_in_circuit = false;
        let mut to_in_circuit = false;
        let mut from_index: usize = 0;
        let mut to_index: usize = 0;

        for (index, circuit) in circuits.iter().enumerate() {
            if !circuit.contains(&path.from) && circuit.contains(&path.to) {
                to_in_circuit = true;
                to_index = index;
            }

            if !circuit.contains(&path.to) && circuit.contains(&path.from) {
                from_in_circuit = true;
                from_index = index;
            }

            if circuit.contains(&path.to) && circuit.contains(&path.from) {
                from_in_circuit = true;
                to_in_circuit = true;
                from_index = index;
                to_index = index;
            }
        }

        if !from_in_circuit && !to_in_circuit {
            circuits.push(vec![path.from, path.to]);
        }

        if !from_in_circuit && to_in_circuit {
            circuits[to_index].push(path.from);
        }

        if from_in_circuit && !to_in_circuit {
            circuits[from_index].push(path.to);
        }

        if from_in_circuit && to_in_circuit && from_index != to_index {
            let mut binding = circuits[to_index].clone();
            circuits[from_index].append(&mut binding);
            circuits.remove(to_index);
        }
    }

    circuits.sort_by(|a, b| b.len().cmp(&a.len()));

    println!("{} {}", "Solution part one:".green().bold(), circuits[0].len() * circuits[1].len() * circuits[2].len());

    let mut last_path: Path = paths[0]; 
    for path in paths[pair_count..paths.len()-1].iter() {
        let mut from_in_circuit = false;
        let mut to_in_circuit = false;
        let mut from_index: usize = 0;
        let mut to_index: usize = 0;

        for (index, circuit) in circuits.iter().enumerate() {
            if !circuit.contains(&path.from) && circuit.contains(&path.to) {
                to_in_circuit = true;
                to_index = index;
                last_path = *path;
            }

            if !circuit.contains(&path.to) && circuit.contains(&path.from) {
                from_in_circuit = true;
                from_index = index;
                last_path = *path;
            }

            if circuit.contains(&path.to) && circuit.contains(&path.from) {
                from_in_circuit = true;
                to_in_circuit = true;
                from_index = index;
            }
        }

        if !from_in_circuit && !to_in_circuit {
            circuits.push(vec![path.from, path.to]);
        }

        if !from_in_circuit && to_in_circuit {
            circuits[to_index].push(path.from);
        }

        if from_in_circuit && !to_in_circuit {
            circuits[from_index].push(path.to);
        }

        if from_in_circuit && to_in_circuit && from_index != to_index {
            let mut binding = circuits[to_index].clone();
            circuits[from_index].append(&mut binding);
            circuits.remove(to_index);
        }
    }

    println!("{} {}", "Solution part two:".green().bold(), last_path.from.x * last_path.to.x);
}
