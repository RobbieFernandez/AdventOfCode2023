use std::{collections::HashSet, env, fs::read_to_string};

#[derive(Debug)]
enum SchematicValue {
    Blank,
    NumberStart(u32, usize),
    NumberContinuation(u32, Position),
    Gear,
    Symbol,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    row: usize,
    column: usize,
}

#[derive(Debug)]
struct Schematic {
    matrix: Vec<Vec<SchematicValue>>,
}

impl Position {
    fn left(&self, _schematic: &Schematic) -> Option<Self> {
        if self.column > 0 {
            Some(Self {
                row: self.row,
                column: self.column - 1,
            })
        } else {
            None
        }
    }

    fn right(&self, schematic: &Schematic) -> Option<Self> {
        if self.column < schematic.width() - 1 {
            Some(Self {
                row: self.row,
                column: self.column + 1,
            })
        } else {
            None
        }
    }

    fn up(&self, _schematic: &Schematic) -> Option<Self> {
        if self.row > 0 {
            Some(Self {
                row: self.row - 1,
                column: self.column,
            })
        } else {
            None
        }
    }

    fn down(&self, schematic: &Schematic) -> Option<Self> {
        if self.row < schematic.height() - 1 {
            Some(Self {
                row: self.row + 1,
                column: self.column,
            })
        } else {
            None
        }
    }
}

impl Schematic {
    fn width(&self) -> usize {
        self.matrix[0].len()
    }

    fn height(&self) -> usize {
        self.matrix.len()
    }

    fn get_neighbours(&self, start_position: &Position, width: usize) -> Vec<Position> {
        let mut neighbours: Vec<Option<Position>> = Vec::new();

        // Add the left edge.
        let left = start_position.left(self);
        if let Some(left) = left {
            neighbours.push(left.up(self));
            neighbours.push(left.down(self));
            neighbours.push(Some(left));
        }

        // Add top/bottom for all middle cells
        for column_offset in 0..width {
            let pos = Position {
                row: start_position.row,
                column: start_position.column + column_offset,
            };
            neighbours.push(pos.up(self));
            neighbours.push(pos.down(self));
        }

        // Add the right edge.
        let end_position = Position {
            row: start_position.row,
            column: start_position.column + width - 1,
        };
        let right = end_position.right(self);
        if let Some(right) = right {
            neighbours.push(right.up(self));
            neighbours.push(right.down(self));
            neighbours.push(Some(right));
        }

        let valid_neightbours = neighbours.into_iter().flatten();

        valid_neightbours.collect()
    }

    fn is_symbol(&self, position: &Position) -> bool {
        let value = &self.matrix[position.row][position.column];
        matches!(value, SchematicValue::Symbol | SchematicValue::Gear)
    }

    fn get_number(&self, position: &Position) -> u32 {
        let value = &self.matrix[position.row][position.column];
        match value {
            SchematicValue::NumberStart(n, _) => *n,
            _ => panic!(
                "Value at position ({}, {}) is not a number.",
                position.row, position.column
            ),
        }
    }
}

fn parse_file(file: &str) -> Schematic {
    let lines = file.trim().lines();

    let mut matrix: Vec<Vec<SchematicValue>> = Vec::new();

    for (i, line) in lines.into_iter().enumerate() {
        let mut pos = 0;
        let chars: Vec<char> = line.chars().collect();
        let mut line_values: Vec<SchematicValue> = Vec::new();

        while pos < chars.len() {
            let next_char = chars[pos];

            if next_char.is_ascii_digit() {
                let mut value: u32 = next_char.to_string().parse().unwrap();
                let start_pos = pos;
                pos += 1;

                // Inner loop to consume the rest of the multi-digit number.
                while pos < chars.len() {
                    let next_char = chars[pos];
                    if next_char.is_ascii_digit() {
                        value *= 10;
                        value += next_char.to_string().parse::<u32>().unwrap();
                        pos += 1;
                    } else {
                        break;
                    }
                }

                let length = pos - start_pos;
                line_values.push(SchematicValue::NumberStart(value, length));
                for _ in 1..length {
                    line_values.push(SchematicValue::NumberContinuation(
                        value,
                        Position {
                            row: i,
                            column: start_pos,
                        },
                    ));
                }
            } else if next_char == '*' {
                line_values.push(SchematicValue::Gear);
                pos += 1;
            } else if next_char == '.' {
                line_values.push(SchematicValue::Blank);
                pos += 1;
            } else {
                line_values.push(SchematicValue::Symbol);
                pos += 1;
            }
        }
        matrix.push(line_values);
    }

    Schematic { matrix }
}

fn solution1(schematic: &Schematic) -> u32 {
    let row_iter = schematic.matrix.iter();

    row_iter
        .enumerate()
        .flat_map(|(i, row)| {
            let number_iter = row
                .iter()
                .enumerate()
                .map(|(j, v)| match v {
                    SchematicValue::NumberStart(n, l) => (j, Some((n, l))),
                    _ => (j, None),
                })
                .filter(|(_j, v)| v.is_some())
                .map(|(j, v)| (j, v.unwrap()));

            let col_iter = number_iter
                .filter(|(j, (_val, width))| {
                    let pos = Position { row: i, column: *j };
                    let neighbours = schematic.get_neighbours(&pos, **width);
                    let mut neighbours = neighbours.iter();
                    neighbours.any(|p| schematic.is_symbol(p))
                })
                .map(|(_j, value)| value.0);
            let col_vec: Vec<&u32> = col_iter.collect();
            col_vec
        })
        .sum()
}

fn solution2(schematic: &Schematic) -> u32 {
    // Find all gears.
    let row_iter = schematic.matrix.iter();

    let gear_pos_iter = row_iter.enumerate().flat_map(|(i, row)| {
        let col_iter = row
            .iter()
            .enumerate()
            .filter(|(_j, v)| matches!(v, SchematicValue::Gear))
            .map(|(j, _v)| Position { row: i, column: j });

        let col_vec: Vec<Position> = col_iter.collect();
        col_vec
    });

    // Collect pairs of part numbers that neighbour gears.
    let gear_neighbour_iter = gear_pos_iter.map(|p| {
        let neighbours = schematic.get_neighbours(&p, 1);
        let number_neighbours = neighbours
            .into_iter()
            .map(|p| {
                let val = &schematic.matrix[p.row][p.column];
                match val {
                    SchematicValue::NumberContinuation(_, pos) => Some(pos.clone()),
                    SchematicValue::NumberStart(_, _) => Some(p),
                    _ => None,
                }
            })
            .flatten();

        // Filter out duplicates
        let number_neighbours: HashSet<Position> = number_neighbours.collect();
        number_neighbours
    });

    // Filter to gears that have exactly 2 neighbouring part numbers.
    let gear_neighbour_iter = gear_neighbour_iter.filter(|neighbours| neighbours.len() == 2);

    // Multiply each pair
    let gear_ratios = gear_neighbour_iter.map(|neighbours| {
        let mut neighbours = neighbours.iter();
        let first = neighbours.next().unwrap();
        let second = neighbours.next().unwrap();

        let first = schematic.get_number(first);
        let second = schematic.get_number(second);

        first * second
    });

    gear_ratios.sum()
}

fn main() {
    let mut args = env::args();
    let filename = args.nth(1).expect("Filename must be given.");
    let input = read_to_string(filename).unwrap();

    let schematic = parse_file(&input);

    let answer1 = solution1(&schematic);
    println!("Solution 1: {}", answer1);

    let answer2 = solution2(&schematic);
    println!("Solution 2: {}", answer2);
}
