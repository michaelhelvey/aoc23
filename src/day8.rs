use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug)]
struct Instructions<'a> {
    directions: Vec<char>,
    map: HashMap<&'a str, (&'a str, &'a str)>,
}

fn parse_key_value_line(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    let (input, key) = alphanumeric1(input)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, (first, second)) = separated_pair(alphanumeric1, tag(", "), alphanumeric1)(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, (key, (first, second))))
}

fn parse_input(input: &str) -> IResult<&str, Instructions<'_>> {
    let (input, first_line) = alpha1(input)?;
    let directions = first_line.chars().collect::<Vec<char>>();

    let (input, lines) = separated_list1(newline, parse_key_value_line)(input.trim())?;

    let map = lines.into_iter().collect::<HashMap<&str, (&str, &str)>>();

    let instructions = Instructions { directions, map };

    Ok((input, instructions))
}

pub fn solve_part_1(input: &str) -> u64 {
    let (_, instructions) = parse_input(input.trim()).unwrap();

    let mut steps = 0;
    let mut direction_idx = 0;
    let mut node = instructions.map.get("AAA").unwrap();

    loop {
        if direction_idx == instructions.directions.len() {
            direction_idx = 0;
        }

        let direction = instructions.directions[direction_idx];
        let next_key = match direction {
            'R' => node.1,
            'L' => node.0,
            _ => panic!("Unknown direction: {}", direction),
        };

        if next_key == "ZZZ" {
            return steps + 1;
        }

        node = instructions.map.get(&next_key).unwrap();

        direction_idx += 1;
        steps += 1;
    }
}

pub fn solve_part_2(input: &str) -> u64 {
    let (_, instructions) = parse_input(input.trim()).unwrap();

    // All nodes that start with A:
    let nodes: Vec<&str> = instructions
        .map
        .iter()
        .filter(|(key, _)| key.ends_with('A'))
        .map(|(key, _)| *key)
        .collect();

    println!("Found {} nodes that start with A", nodes.len());

    // Store the number of steps it takes to get each node to Z:
    let mut state: HashMap<&str, u64> = HashMap::new();

    for node in nodes {
        let mut steps = 0;
        let mut current_node = node;

        let mut direction_idx = 0;

        loop {
            if current_node.ends_with('Z') {
                break;
            }

            if direction_idx == instructions.directions.len() {
                direction_idx = 0;
            }

            let direction = instructions.directions[direction_idx];

            let (left, right) = instructions.map.get(current_node).unwrap();

            current_node = match direction {
                'R' => right,
                'L' => left,
                _ => panic!("Unknown direction: {}", direction),
            };

            steps += 1;
            direction_idx += 1;
        }

        state.insert(node, steps);
    }

    // they "lineup" when they all divide evenly into the same number of steps:
    // this is just the least common multiple of all the steps
    let nums: Vec<_> = state.values().copied().collect();
    least_common_multiple(nums)
}

fn least_common_multiple(nums: Vec<u64>) -> u64 {
    let result: u64 = nums.iter().fold(1, |acc, &num| acc * num / gcd(acc, num));
    result
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
    "#;

    #[test]
    fn test_parse_input() {
        let (_, instructions) = parse_input(INPUT.trim()).unwrap();
        assert_eq!(instructions.directions, vec!['R', 'L']);
        assert_eq!(instructions.map.get("AAA"), Some(&("BBB", "CCC")));
    }

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(INPUT), 2);
    }

    #[test]
    fn test_part_2() {
        let input = r#"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
        "#;
        assert_eq!(solve_part_2(input.trim()), 6);
    }
}
