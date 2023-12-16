#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthToEast,
    NorthToWest,
    SouthToWest,
    SouthToEast,
    Ground,
    StartingPosition,
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthToEast,
            'J' => Self::NorthToWest,
            '7' => Self::SouthToWest,
            'F' => Self::SouthToEast,
            '.' => Self::Ground,
            'S' => Self::StartingPosition,
            c => panic!("Unrecognized character in input: {}", c),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Node {
    location: Location,
    typ: Pipe,
    c: char,
}

impl Node {
    fn outputs(&self, graph: &Graph) -> (Location, Location) {
        match self.typ {
            Pipe::Vertical => {
                let input = Location::from((self.location.row + 1, self.location.col));
                let output = Location::from((self.location.row - 1, self.location.col));

                (input, output)
            }
            Pipe::Horizontal => {
                let input = Location::from((self.location.row, self.location.col + 1));
                let output = Location::from((self.location.row, self.location.col - 1));

                (input, output)
            }
            Pipe::NorthToEast => {
                let input = Location::from((self.location.row - 1, self.location.col));
                let output = Location::from((self.location.row, self.location.col + 1));

                (input, output)
            }
            Pipe::NorthToWest => {
                let input = Location::from((self.location.row - 1, self.location.col));
                let output = Location::from((self.location.row, self.location.col - 1));

                (input, output)
            }
            Pipe::SouthToWest => {
                let input = Location::from((self.location.row + 1, self.location.col));
                let output = Location::from((self.location.row, self.location.col - 1));

                (input, output)
            }
            Pipe::SouthToEast => {
                let input = Location::from((self.location.row + 1, self.location.col));
                let output = Location::from((self.location.row, self.location.col + 1));

                (input, output)
            }
            Pipe::StartingPosition => {
                let neighors = vec![
                    // the four cardinal directions
                    Location::from((self.location.row + 1, self.location.col)),
                    Location::from((self.location.row - 1, self.location.col)),
                    Location::from((self.location.row, self.location.col + 1)),
                    Location::from((self.location.row, self.location.col - 1)),
                    // and now the diagonals:
                    Location::from((self.location.row + 1, self.location.col - 1)),
                    Location::from((self.location.row + 1, self.location.col + 1)),
                    Location::from((self.location.row - 1, self.location.col + 1)),
                    Location::from((self.location.row - 1, self.location.col - 1)),
                ];

                let mut outputs = neighors
                    .iter()
                    .filter_map(|location| graph.get(location))
                    .filter(|node| {
                        if node.typ == Pipe::Ground {
                            return false;
                        }

                        let (left, right) = graph.get_node_outputs(node);
                        // if either left or right include our position, then there is a connection
                        if left
                            .map(|node| node.location == self.location)
                            .unwrap_or(false)
                            || right
                                .map(|node| node.location == self.location)
                                .unwrap_or(false)
                        {
                            return true;
                        }

                        false
                    })
                    .map(|node| node.location.clone())
                    .collect::<Vec<Location>>();

                let empty_position = Location::from((-1, -1));

                (
                    outputs.pop().unwrap_or(empty_position.clone()),
                    outputs.pop().unwrap_or(empty_position),
                )
            }
            typ => panic!("Cannot traverse pipe of type {:?}", typ),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Location {
    row: i32,
    col: i32,
}

impl From<(i32, i32)> for Location {
    fn from(value: (i32, i32)) -> Self {
        Self {
            row: value.0,
            col: value.1,
        }
    }
}

struct Graph {
    rows: Vec<Vec<Node>>,
    start: Location,
}

impl Graph {
    fn from_input(input: &str) -> Self {
        let mut start: Location = (0, 0).into();

        Self {
            rows: input
                .lines()
                .enumerate()
                .map(|(row_idx, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(col_idx, c)| {
                            let pipe_type: Pipe = c.into();

                            if pipe_type == Pipe::StartingPosition {
                                start = (row_idx as i32, col_idx as i32).into();
                            }

                            Node {
                                location: (row_idx as i32, col_idx as i32).into(),
                                typ: pipe_type,
                                c,
                            }
                        })
                        .collect()
                })
                .collect(),
            start: start.into(),
        }
    }

    fn get(&self, location: &Location) -> Option<&Node> {
        if location.row < 0 || location.col < 0 {
            return None;
        }

        if let Some(row) = self.rows.get(location.row as usize) {
            return row.get(location.col as usize);
        }

        None
    }

    fn get_node_outputs(&self, node: &Node) -> (Option<&Node>, Option<&Node>) {
        let (left, right) = node.outputs(self);

        (self.get(&left), self.get(&right))
    }

    fn get_loop(&self, start: &Node) -> Vec<Location> {
        let (left, right) = self.get_node_outputs(&start);
        let left_path = self.rec_get_loop_path(left, Some(start), 0);

        if let Some(left_path) = left_path {
            return left_path;
        }

        let right_path = self.rec_get_loop_path(right, Some(start), 0);
        if let Some(right_path) = right_path {
            return right_path;
        }

        panic!("Expected to find a loop somewhere")
    }

    fn rec_get_loop_path(
        &self,
        current: Option<&Node>,
        previous: Option<&Node>,
        _depth: usize,
    ) -> Option<Vec<Location>> {
        // println!("current: {:?}", current);

        if let Some(node) = current {
            // base cases:
            match node.typ {
                Pipe::StartingPosition => {
                    return Some(vec![node.location.clone()]);
                }
                Pipe::Ground => return None,
                _ => {}
            }

            let (left, right) = self.get_node_outputs(node);

            if let Some(left) = left {
                let next_is_previous = previous
                    .map(|node| node.location == left.location)
                    .unwrap_or(false);

                if !next_is_previous {
                    let left_path = self.rec_get_loop_path(Some(left), Some(node), _depth + 1);
                    if let Some(mut left_path) = left_path {
                        let mut result = vec![node.location.clone()];
                        result.append(&mut left_path);
                        return Some(result);
                    }
                }
            }

            if let Some(right) = right {
                let next_is_previous = previous
                    .map(|node| node.location == right.location)
                    .unwrap_or(false);

                if !next_is_previous {
                    let right_path = self.rec_get_loop_path(Some(right), Some(node), _depth + 1);
                    if let Some(mut right_path) = right_path {
                        let mut result = vec![node.location.clone()];
                        result.append(&mut right_path);
                        return Some(result);
                    }
                }
            }
        }

        None
    }
}

pub fn solve_part_1(input: &str) -> usize {
    let graph = Graph::from_input(input);
    let loop_path = graph.get_loop(graph.get(&graph.start).unwrap());

    loop_path.len() / 2
}

pub fn solve_part_2(input: &str) -> usize {
    // step 1: iterate around the loop so you know where all the possible candidates are.
    // step 2: go through the candidates to find which ones are in a "potentially contained area"...
    // an area can be found by whenever you find a pipe that touches a pipe that you previously
    // traversed.  the path from <previously found...current> contains a potential area
    // step 3: for each contained square, find if it can get out by just checking the pipe
    // edges...i.e. if you find a parallel pipe to the direction you're looking, just follow that
    // path to see if you can eventually get out
    // step 4: each time you enounter one that can get out, mark that whole area (that you identified in step 2) as "escapable"
    // step 5...take the sum of all the unmarked areas and there's your answer.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = r#"
.....
.S-7.
.|.|.
.L-J.
.....
    "#;

    #[test]
    fn test_example_input_1() {
        let input = r#"
.....
.S-7.
.|.|.
.L-J.
.....
    "#;

        assert_eq!(solve_part_1(input.trim()), 4);
    }
}
