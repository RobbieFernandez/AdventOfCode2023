use std::{env, fs::read_to_string};

#[derive(Debug)]
struct Turn {
    red_count: u32,
    green_count: u32,
    blue_count: u32,
}

#[derive(Debug)]
struct Game {
    id: u32,
    turns: Vec<Turn>,
}

impl Game {
    fn new(id: u32) -> Self {
        Self {
            id,
            turns: Vec::new(),
        }
    }

    fn is_possible(&self, red_total: u32, green_total: u32, blue_total: u32) -> bool {
        self.turns
            .iter()
            .all(|t| t.is_possible(red_total, green_total, blue_total))
    }

    fn power(&self) -> u32 {
        let min_red = self.turns.iter().map(|t| t.red_count).max().unwrap_or(0);
        let min_green = self.turns.iter().map(|t| t.green_count).max().unwrap_or(0);
        let min_blue = self.turns.iter().map(|t| t.blue_count).max().unwrap_or(0);

        min_red * min_blue * min_green
    }
}

impl Turn {
    fn new() -> Self {
        Self {
            red_count: 0,
            green_count: 0,
            blue_count: 0,
        }
    }

    fn is_possible(&self, red_total: u32, green_total: u32, blue_total: u32) -> bool {
        self.red_count <= red_total
            && self.green_count <= green_total
            && self.blue_count <= blue_total
    }
}

fn parse_line(line: &str) -> Game {
    let mut split = line.split(":");
    let head = split.next().unwrap();
    let tail = split.next().unwrap();

    let head = head.split(" ");
    let head = head.last().unwrap();
    let id: u32 = head.parse().unwrap();

    let mut game = Game::new(id);

    let turns = tail.split(";");

    for turn_line in turns {
        let counts = turn_line.split(", ");
        let mut turn = Turn::new();

        for substr in counts {
            let mut substr = substr.trim().split(" ");
            let count: u32 = substr.next().unwrap().parse().unwrap();
            let color = substr.next().unwrap();

            match color {
                "red" => turn.red_count = count,
                "blue" => turn.blue_count = count,
                "green" => turn.green_count = count,
                _ => panic!("Unkown color {}", color),
            }
        }

        game.turns.push(turn);
    }

    game
}

fn solution1(games: &Vec<Game>) -> u32 {
    let games = games.iter().filter(|g| g.is_possible(12, 13, 14));
    games.map(|g| g.id).sum()
}

fn solution2(games: &Vec<Game>) -> u32 {
    games.iter().map(|g| g.power()).sum()
}

fn main() {
    let mut args = env::args();
    let filename = args.nth(1).expect("Filename must be given.");

    let input = read_to_string(filename).unwrap();
    let input = input.trim().lines();
    let games = input.map(|l| parse_line(l));
    let games: Vec<Game> = games.collect();

    let answer1 = solution1(&games);
    println!("Solution1: {}", answer1);

    let answer2 = solution2(&games);
    println!("Solution2: {}", answer2);
}
