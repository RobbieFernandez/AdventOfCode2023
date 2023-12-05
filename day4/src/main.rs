use std::collections::HashSet;
use std::env;
use std::fs::read_to_string;

struct Card {
    winning_numbers: HashSet<u32>,
    your_numbers: HashSet<u32>,
}

impl Card {
    fn points(&self) -> u32 {
        let num_winning: u32 = self.num_matches().try_into().unwrap();
        if num_winning > 0 {
            2u32.pow(num_winning - 1)
        } else {
            0
        }
    }

    fn num_matches(&self) -> usize {
        let your_winning_numbers = self.winning_numbers.intersection(&self.your_numbers);
        your_winning_numbers.count()
    }
}

fn parse_input(filename: &str) -> Vec<Card> {
    let lines = read_to_string(filename).unwrap();
    let lines = lines.lines();

    let cards = lines.map(|l| {
        let l = l.split(':').nth(1).unwrap();
        let mut split = l.split('|');

        let winning_numbers = split.next().unwrap();
        let winning_numbers = winning_numbers
            .split(' ')
            .filter(|n| n != &"")
            .map(|n| n.trim().parse::<u32>().expect("Invalid number"));
        let winning_numbers: HashSet<u32> = winning_numbers.collect();

        let your_numbers = split.next().unwrap();
        let your_numbers = your_numbers
            .split(' ')
            .filter(|n| n != &"")
            .map(|n| n.trim().parse::<u32>().expect("Invalid number"));
        let your_numbers: HashSet<u32> = your_numbers.collect();

        Card {
            winning_numbers,
            your_numbers,
        }
    });

    cards.collect()
}

fn solution1(cards: &[Card]) -> u32 {
    cards.iter().map(|c| c.points()).sum()
}

fn solution2(cards: &[Card]) -> u32 {
    let mut card_counts: Vec<u32> = cards.iter().map(|_| 1).collect();

    for (i, card) in cards.iter().enumerate() {
        let card_count = card_counts[i];
        let matches = card.num_matches();

        for j in 1..=matches {
            let j: usize = j;
            card_counts[i + j] += card_count;
        }
    }

    card_counts.iter().sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let cards = parse_input(filename);

    let answer1 = solution1(&cards);
    println!("Solution 1: {}", answer1);

    let answer2 = solution2(&cards);
    println!("Solution 2: {}", answer2);
}
