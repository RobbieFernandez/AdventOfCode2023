use std::env;
use std::fs::read_to_string;

fn parse_input(filename: &str) -> Vec<Vec<i64>> {
    let input = read_to_string(filename).expect("Error reading file");
    let lines = input.lines();

    lines
        .map(|l| {
            let line = l.split(' ');
            line.map(|c| c.parse::<i64>().expect("invalid number"))
                .collect::<Vec<i64>>()
        })
        .collect()
}

fn predict_next(numbers: &[i64]) -> i64 {
    if numbers.iter().all(|n| *n == 0) {
        0
    } else {
        let differences: Vec<i64> = numbers
            .windows(2)
            .map(|window| {
                let first = window[0];
                let second = window[1];
                second - first
            })
            .collect();

        numbers.last().unwrap() + predict_next(&differences)
    }
}

fn predict_previous(numbers: &[i64]) -> i64 {
    if numbers.iter().all(|n| *n == 0) {
        0
    } else {
        let differences: Vec<i64> = numbers
            .windows(2)
            .map(|window| {
                let first = window[0];
                let second = window[1];
                second - first
            })
            .collect();

        numbers.first().unwrap() - predict_previous(&differences)
    }
}

fn solution1(number_lists: &[Vec<i64>]) -> i64 {
    number_lists
        .iter()
        .map(|numbers| predict_next(numbers))
        .sum()
}

fn solution2(number_lists: &[Vec<i64>]) -> i64 {
    number_lists
        .iter()
        .map(|numbers| predict_previous(numbers))
        .sum()
}

fn main() {
    let mut args = env::args();
    let filename = args.nth(1).expect("Filename must be given.");
    let input = parse_input(&filename);

    let answer1 = solution1(&input);
    println!("Solution 1: {}", answer1);

    let answer2 = solution2(&input);
    println!("Solution 2: {}", answer2);
}
