use std::env;
use std::fs::read_to_string;

fn solution1(filename: &str) -> u32 {
    let lines = read_to_string(filename).unwrap();
    let lines = lines.trim().lines();

    let numbers = lines.map(|l| {
        let mut digits = l.chars().filter(|c| c.is_ascii_digit());
        let first = digits.next().unwrap();
        let last = digits.last().unwrap_or(first);
        let number = format!("{}{}", first, last);
        let number: u32 = number.parse().unwrap();
        number
    });

    numbers.sum()
}

fn parse_number(value: &str) -> Option<u32> {
    if value.len() == 1 {
        let value_char: char = value.chars().next().unwrap();
        if value_char.is_ascii_digit() {
            Some(value.parse().unwrap())
        } else {
            None
        }
    } else {
        match value {
            "one" => Some(1),
            "two" => Some(2),
            "three" => Some(3),
            "four" => Some(4),
            "five" => Some(5),
            "six" => Some(6),
            "seven" => Some(7),
            "eight" => Some(8),
            "nine" => Some(9),
            _ => None,
        }
    }
}

fn solution2(filename: &str) -> u32 {
    let search_values = vec![
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine",
    ];

    let lines = read_to_string(filename).unwrap();
    let lines = lines.trim().lines();

    let numbers = lines.map(|l| {
        // Find all numbers in the line.

        let mut window_start = 0;
        let mut window_end = 0;
        let mut numbers: Vec<u32> = Vec::new();

        while window_end < l.len() {
            let current_str = &l[window_start..=window_end];
            // Is the string already in the number list?
            if search_values.contains(&current_str) {
                let number = parse_number(current_str).unwrap();
                numbers.push(number);
                window_start += 1;
            } else {
                // Are we building towards a number in the list?
                if search_values.iter().any(|n| n.starts_with(current_str)) {
                    window_end += 1;
                } else {
                    window_start += 1;
                }
            }

            if window_start > window_end {
                window_end = window_start;
            }
        }

        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();
        let number = format!("{}{}", first, last);
        let number: u32 = number.parse().unwrap();
        number
    });

    numbers.sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let answer1 = solution1(filename);
    println!("{}", answer1);

    let answer2 = solution2(filename);
    println!("{}", answer2);
}
