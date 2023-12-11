use std::{collections::HashSet, env, fs::read_to_string, iter};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
    Unknown,
}

type Pipe = Option<(Direction, Direction)>;
type Position = (usize, usize);

struct PipeWalker<'a> {
    start_position: Position,
    pipe_matrix: &'a [Vec<Pipe>],
    position: Position,
    path: Vec<Direction>,
    finished: bool,
}

impl Direction {
    fn move_from_pipe(&self, from: &Position) -> Option<Position> {
        match self {
            Self::Left => {
                if from.1 > 0 {
                    Some((from.0, from.1 - 1))
                } else {
                    None
                }
            }
            Self::Right => Some((from.0, from.1 + 1)),
            Self::Down => Some((from.0 + 1, from.1)),
            Self::Up => {
                if from.0 > 0 {
                    Some((from.0 - 1, from.1))
                } else {
                    None
                }
            }
            _ => panic!("Invalid movement"),
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            _ => panic!("Invalid movement"),
        }
    }
}

impl<'a> PipeWalker<'a> {
    fn new(
        pipe_matrix: &'a [Vec<Pipe>],
        position: &Position,
        initial_direction: Direction,
    ) -> Self {
        let starting_pipe = &pipe_matrix[position.0][position.1].expect("Must start at a pipe.");

        let direction_to_ignore = if starting_pipe.0 == initial_direction {
            starting_pipe.1
        } else {
            starting_pipe.0
        };

        Self {
            pipe_matrix,
            start_position: position.clone(),
            position: position.clone(),
            path: vec![direction_to_ignore.opposite()],
            finished: false,
        }
    }
}

impl<'a> Iterator for PipeWalker<'a> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let current_pipe = self.pipe_matrix[self.position.0][self.position.1]
            .expect("path can only contain pipes");
        let came_from = self.path.last().unwrap().opposite();

        let next_direction = if current_pipe.0 == came_from {
            current_pipe.1
        } else {
            current_pipe.0
        };

        self.path.push(next_direction);

        let next_pos = next_direction.move_from_pipe(&self.position).unwrap();
        self.position = next_pos;

        if self.position == self.start_position {
            self.finished = true;
        }

        Some(next_pos)
    }
}

fn parse_pipe_char(c: char) -> Pipe {
    match c {
        '|' => Some((Direction::Up, Direction::Down)),
        '-' => Some((Direction::Left, Direction::Right)),
        'L' => Some((Direction::Up, Direction::Right)),
        'J' => Some((Direction::Up, Direction::Left)),
        '7' => Some((Direction::Down, Direction::Left)),
        'F' => Some((Direction::Down, Direction::Right)),
        'S' => Some((Direction::Unknown, Direction::Unknown)),
        '.' => None,
        _ => panic!("Unknown character: {}", c),
    }
}

fn parse_input(filename: &str) -> ((usize, usize), Vec<Vec<Pipe>>) {
    let input = read_to_string(filename).expect("Could not read file.");
    let lines = input.lines();

    let mut matrix: Vec<Vec<Pipe>> = Vec::new();

    let mut start = (0usize, 0usize);

    for (i, line) in lines.enumerate() {
        let mut pipes: Vec<Pipe> = Vec::new();

        for (j, c) in line.chars().enumerate() {
            let pipe = parse_pipe_char(c);

            if let Some((Direction::Unknown, _)) = pipe {
                start = (i, j);
            }

            pipes.push(pipe);
        }

        matrix.push(pipes);
    }

    (start, matrix)
}

fn solve_starting_pipe(
    start_position: &(usize, usize),
    pipe_matrix: &[Vec<Pipe>],
) -> Vec<Vec<Pipe>> {
    let mut connected_directions: Vec<Direction> = Vec::new();

    for direction in vec![
        Direction::Up,
        Direction::Down,
        Direction::Right,
        Direction::Left,
    ] {
        let neighbour = direction.move_from_pipe(start_position);

        if let Some(neighbour) = neighbour {
            let neighbour_pipe = &pipe_matrix[neighbour.0][neighbour.1];

            if let Some(neighbour_pipe) = neighbour_pipe {
                let opposite_direction = direction.opposite();

                if neighbour_pipe.0 == opposite_direction || neighbour_pipe.1 == opposite_direction
                {
                    connected_directions.push(direction);
                }
            }
        }
    }

    // Should be exactly 2 directions.
    if connected_directions.len() != 2 {
        panic!(
            "Could not find starting pipe configuration. Found directions {:?}",
            connected_directions
        );
    }

    let starting_pipe = (connected_directions[0], connected_directions[1]);

    // Copy the matrix, so we can mutate it with the new starting pipe.
    let mut pipe_matrix = pipe_matrix.to_vec();
    pipe_matrix[start_position.0][start_position.1] = Some(starting_pipe);

    pipe_matrix
}

fn inflate_matrix(pipe_matrix: &[Vec<Pipe>]) -> Vec<Vec<Pipe>> {
    let mut new_matrix: Vec<Vec<Pipe>> = Vec::new();

    let num_rows = pipe_matrix.len();

    for (i, row) in pipe_matrix.iter().enumerate() {
        let num_cols = row.len();
        let is_last_row = i + 1 == num_rows;

        new_matrix.push(Vec::new());

        if !is_last_row {
            new_matrix.push(Vec::new());
        }

        for (j, value) in row.iter().enumerate() {
            let is_last_col = j + 1 == num_cols;

            new_matrix[i * 2].push(*value);

            if !is_last_col {
                let right_value = &pipe_matrix[i][j + 1];

                let is_connected = matches!(right_value, Some((Direction::Left, _)))
                    || matches!(right_value, Some((_, Direction::Left)));

                if is_connected {
                    new_matrix[i * 2].push(Some((Direction::Left, Direction::Right)));
                } else {
                    new_matrix[i * 2].push(None);
                }
            }

            if !is_last_row {
                let bottom_value = &pipe_matrix[i + 1][j];

                let is_connected = matches!(bottom_value, Some((Direction::Up, _)))
                    || matches!(bottom_value, Some((_, Direction::Up)));

                if is_connected {
                    new_matrix[i * 2 + 1].push(Some((Direction::Up, Direction::Down)));
                } else {
                    new_matrix[i * 2 + 1].push(None);
                }
                new_matrix[i * 2 + 1].push(None);
            }
        }
    }

    new_matrix
}

fn flood(
    start_position: &(usize, usize),
    inflated_pipe_matrix: &[Vec<Pipe>],
    pipes_in_loop: &HashSet<(usize, usize)>,
    mut in_loop: HashSet<(usize, usize)>,
    mut outside_loop: HashSet<(usize, usize)>,
) -> (HashSet<(usize, usize)>, HashSet<(usize, usize)>) {
    let mut to_visit: Vec<(usize, usize)> = vec![*start_position];
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    seen.insert(*start_position);

    let num_rows = inflated_pipe_matrix.len();
    let num_cols = inflated_pipe_matrix[0].len();

    while to_visit.len() > 0 {
        let next_node = to_visit.pop().unwrap();

        // If we reach a node that is known to be in/out of the loop.
        // Then everything we've seen already is also in/out of the loop.
        if in_loop.contains(&next_node) {
            in_loop.extend(seen);
            return (in_loop, outside_loop);
        }

        if outside_loop.contains(&next_node) {
            outside_loop.extend(seen);
            return (in_loop, outside_loop);
        }

        let is_edge = next_node.0 == 0
            || next_node.1 == 0
            || next_node.0 + 1 >= num_rows
            || next_node.1 + 1 >= num_cols;

        // If we hit an edge then we are out of the loop.
        if is_edge {
            outside_loop.extend(seen);
            return (in_loop, outside_loop);
        }

        let neighbours = vec![
            (next_node.0 + 1, next_node.1),
            (next_node.0 - 1, next_node.1),
            (next_node.0, next_node.1 + 1),
            (next_node.0, next_node.1 - 1),
        ];

        for neighbour in neighbours {
            if !pipes_in_loop.contains(&neighbour) {
                if seen.insert(neighbour) {
                    to_visit.push(neighbour);
                }
            }
        }
    }

    in_loop.extend(seen);
    (in_loop, outside_loop)
}

fn solution1(start_position: &(usize, usize), pipe_matrix: &[Vec<Pipe>]) -> u32 {
    // Figure out configuration of starting pipe.
    let pipe_matrix = solve_starting_pipe(start_position, pipe_matrix);
    let starting_pipe = pipe_matrix[start_position.0][start_position.1].unwrap();

    // Now follow both paths from starting_pipe, until they collide.
    let path1 = PipeWalker::new(&pipe_matrix, start_position, starting_pipe.0);
    let path2 = PipeWalker::new(&pipe_matrix, start_position, starting_pipe.1);

    let paths = iter::zip(path1, path2);

    for (i, (pos1, pos2)) in paths.enumerate() {
        if pos1 == pos2 {
            return i as u32 + 1;
        }
    }

    panic!("No solution found")
}

fn solution2(start_position: &(usize, usize), pipe_matrix: &[Vec<Pipe>]) -> usize {
    let pipe_matrix = solve_starting_pipe(start_position, pipe_matrix);
    let starting_pipe = pipe_matrix[start_position.0][start_position.1].unwrap();

    let inflated_pipe_matrix = inflate_matrix(&pipe_matrix);
    let pipe_loop = PipeWalker::new(
        &inflated_pipe_matrix,
        &(start_position.0 * 2, start_position.1 * 2),
        starting_pipe.0,
    );

    let nodes_in_main_loop: HashSet<(usize, usize)> = HashSet::from_iter(pipe_loop);

    let num_rows = inflated_pipe_matrix.len();
    let num_cols = inflated_pipe_matrix[0].len();

    let mut nodes_in_loop: HashSet<(usize, usize)> = HashSet::new();
    let mut nodes_outside_loop: HashSet<(usize, usize)> = HashSet::new();

    for pipe_node in nodes_in_main_loop.iter() {
        let mut neighbours: Vec<(usize, usize)> = Vec::new();

        let is_top_edge = pipe_node.0 == 0;
        let is_bottom_edge = pipe_node.0 == num_rows - 1;
        let is_left_edge = pipe_node.1 == 0;
        let is_right_edge = pipe_node.1 == num_cols - 1;

        if !is_top_edge {
            neighbours.push((pipe_node.0 - 1, pipe_node.1));

            if !is_left_edge {
                neighbours.push((pipe_node.0 - 1, pipe_node.1 - 1));
            }

            if !is_right_edge {
                neighbours.push((pipe_node.0 - 1, pipe_node.1 + 1));
            }
        }

        if !is_bottom_edge {
            neighbours.push((pipe_node.0 + 1, pipe_node.1));

            if !is_left_edge {
                neighbours.push((pipe_node.0 + 1, pipe_node.1 - 1));
            }

            if !is_right_edge {
                neighbours.push((pipe_node.0 + 1, pipe_node.1 + 1));
            }
        }

        if !is_left_edge {
            neighbours.push((pipe_node.0, pipe_node.1 - 1));
        }

        if !is_right_edge {
            neighbours.push((pipe_node.0, pipe_node.1 + 1));
        }

        for neighbour in neighbours {
            if !nodes_in_main_loop.contains(&neighbour) {
                (nodes_in_loop, nodes_outside_loop) = flood(
                    &neighbour,
                    &inflated_pipe_matrix,
                    &nodes_in_main_loop,
                    nodes_in_loop,
                    nodes_outside_loop,
                );
            }
        }
    }

    nodes_in_loop
        .iter()
        .filter(|(row, col)| row % 2 == 0 && col % 2 == 0)
        .count()
}

fn main() {
    let filename = env::args()
        .nth(1)
        .expect("filename must be given as first argument");

    let (start_position, pipe_matrix) = parse_input(&filename);

    let answer1 = solution1(&start_position, &pipe_matrix);
    println!("Solution 1: {}", answer1);

    let answer2 = solution2(&start_position, &pipe_matrix);
    println!("Solution 2: {}", answer2);
}
