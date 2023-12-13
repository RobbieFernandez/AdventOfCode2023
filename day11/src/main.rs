use std::{collections::HashSet, env, fs::read_to_string};

#[derive(Debug, Clone)]
enum Space {
    Empty,
    Galaxy(usize),
}

#[derive(Debug, Clone)]
struct Universe {
    matrix: Vec<Vec<Space>>,
    expanded_rows: HashSet<usize>,
    expanded_cols: HashSet<usize>,
}

impl Space {
    fn from_char(c: char, id: usize) -> Self {
        match c {
            '.' => Self::Empty,
            '#' => Self::Galaxy(id),
            _ => panic!("Unknown char: {}", c),
        }
    }
}

impl Universe {
    fn expand(&mut self) {
        // Add extra rows for any rows that have no galaxy.

        let row_count = self.matrix.len();
        let col_count = self.matrix[0].len();

        // Count backwards so we don't mess up the indices.
        for i in (0..row_count).rev() {
            let row = &self.matrix[i];

            if row.iter().all(|s| matches!(s, Space::Empty)) {
                self.expanded_rows.insert(i);
            }
        }

        for i in (0..col_count).rev() {
            if self.matrix.iter().all(|row| matches!(row[i], Space::Empty)) {
                self.expanded_cols.insert(i);
            }
        }
    }

    fn get_galaxy_positions(&self) -> Vec<(usize, usize)> {
        self.matrix
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(j, val)| {
                        if matches!(val, Space::Empty) {
                            None
                        } else {
                            Some((i, j))
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

fn parse_input(filename: &str) -> Universe {
    let input = read_to_string(filename).expect("Error reading file");
    let mut universe_matrix: Vec<Vec<Space>> = Vec::new();

    let lines = input.lines();

    for line in lines {
        let mut id = 0;
        let row: Vec<Space> = line
            .chars()
            .map(|c| {
                let space = Space::from_char(c, id);

                if let Space::Galaxy(_) = space {
                    id += 1;
                }

                space
            })
            .collect();

        universe_matrix.push(row);
    }

    Universe {
        matrix: universe_matrix,
        expanded_cols: HashSet::new(),
        expanded_rows: HashSet::new(),
    }
}

fn solution(universe: &Universe, expansion_amount: usize) -> usize {
    let mut universe = universe.clone();
    universe.expand();

    let galaxy_positions = universe.get_galaxy_positions();

    (0..galaxy_positions.len())
        .flat_map(|i| {
            (i..galaxy_positions.len())
                .map(|j| {
                    let start = &galaxy_positions[i];
                    let end = &galaxy_positions[j];

                    let mut dist = 0;

                    let rows = if end.0 > start.0 {
                        start.0..end.0
                    } else {
                        end.0..start.0
                    };

                    for row in rows {
                        dist += if universe.expanded_rows.contains(&row) {
                            expansion_amount
                        } else {
                            1
                        }
                    }

                    let cols = if end.1 > start.1 {
                        start.1..end.1
                    } else {
                        end.1..start.1
                    };

                    for col in cols {
                        dist += if universe.expanded_cols.contains(&col) {
                            expansion_amount
                        } else {
                            1
                        }
                    }
                    dist
                })
                .collect::<Vec<_>>()
        })
        .sum()
}

fn solution1(universe: &Universe) -> usize {
    solution(universe, 2)
}

fn solution2(universe: &Universe) -> usize {
    solution(universe, 1_000_000)
}

fn main() {
    let filename = env::args().nth(1).expect("filename must be given");
    let universe = parse_input(&filename);

    let answer1 = solution1(&universe);
    println!("Solution1: {}", answer1);

    let answer2 = solution2(&universe);
    println!("Solution2: {}", answer2);
}
