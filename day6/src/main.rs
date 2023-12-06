use std::env;
use std::fs::read_to_string;
use std::iter::zip;

#[derive(Debug)]
struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn winning_range(&self) -> (u64, u64) {
        // Find roots for quadratic:
        //  y = -x^2 + (Time)x -(Record + 1)
        let a = -1f64;
        let b: f64 = self.time as f64;

        let c: f64 = self.record as f64;
        let c = (1.0 + c) * -1.0;

        let d = b.powi(2) - (4f64 * a * c);

        let root1 = (-b + d.sqrt()) / (2f64 * a);
        let root2 = (-b - d.sqrt()) / (2f64 * a);

        let root1: u64 = root1.ceil() as u64;
        let root2: u64 = root2.floor() as u64;

        (root1, root2)
    }
}

fn parse_line(line: &str) -> Vec<u64> {
    let mut line = line.split(':');
    let input = line.nth(1).unwrap();
    let input = input.trim();
    input
        .split(' ')
        .filter(|x| x != &"")
        .map(|val| val.parse::<u64>().expect("not a valid number"))
        .collect()
}

fn parse_input(filename: &str) -> Vec<Race> {
    let input = read_to_string(filename).expect("File could not be read");
    let mut input = input.trim().lines();
    let first_line = input.next().expect("Could not read first line");
    let second_line = input.next().expect("Could not read second line");

    let times = parse_line(first_line);
    let records = parse_line(second_line);

    zip(times, records)
        .map(|(time, record)| Race { time, record })
        .collect()
}

fn solution1(races: &[Race]) -> u64 {
    races
        .iter()
        .map(|race| {
            let (start, end) = race.winning_range();
            end - start + 1
        })
        .reduce(|acc, next| acc * next)
        .unwrap()
}

fn solution2(races: &[Race]) -> u64 {
    // Create 1 big race from all the smaller ones
    let mut time_str = String::new();
    let mut record_str = String::new();

    for race in races {
        let next_time_str = format!("{}", race.time);
        let next_record_str = format!("{}", race.record);

        time_str.push_str(&next_time_str);
        record_str.push_str(&next_record_str);
    }

    let total_time: u64 = time_str.parse().unwrap();
    let total_record: u64 = record_str.parse().unwrap();

    let race = Race {
        time: total_time,
        record: total_record,
    };

    let (start, end) = race.winning_range();
    end - start + 1
}

fn main() {
    let mut args = env::args();
    let filename = args.nth(1).expect("Filename must be given.");

    let races = parse_input(&filename);

    let answer1 = solution1(&races);
    println!("Solution 1: {}", answer1);

    let answer2 = solution2(&races);
    println!("Solution 2: {}", answer2);
}
