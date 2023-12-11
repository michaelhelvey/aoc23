use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use color_eyre::owo_colors::colors::xterm::VistaBlue;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, newline},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
struct Instructions<'a> {
    directions: Vec<char>,
    map: HashMap<&'a str, (&'a str, &'a str)>,
}

fn parse_key_value_line<'a>(input: &'a str) -> IResult<&'a str, (&'a str, (&'a str, &'a str))> {
    let (input, key) = alphanumeric1(input)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, (first, second)) = separated_pair(alphanumeric1, tag(", "), alphanumeric1)(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, (key, (first, second))))
}

fn parse_input<'a>(input: &'a str) -> IResult<&'a str, Instructions<'a>> {
    let (input, first_line) = alpha1(input)?;
    let directions = first_line.chars().collect::<Vec<char>>();

    let (input, lines) = separated_list1(newline, parse_key_value_line)(input.trim())?;

    let map = lines.into_iter().collect::<HashMap<&str, (&str, &str)>>();

    let instructions = Instructions { directions, map };

    Ok((input, instructions))
}

fn get_left(key: &'static str, instructions: &Instructions<'static>) -> Option<&'static str> {
    let (left, _) = instructions.map.get(key).unwrap();
    Some(left)
}

fn get_right(key: &'static str, instructions: &Instructions<'static>) -> Option<&'static str> {
    let (_, right) = instructions.map.get(key).unwrap();
    Some(right)
}

pub fn solve_part_2(input: &'static str) -> u64 {
    let (_, instructions) = parse_input(input.trim()).unwrap();

    let mut nodes: Vec<&str> = instructions
        .map
        .iter()
        .filter(|(key, _)| key.ends_with('A'))
        .map(|(key, _)| *key)
        .collect();

    let mut steps = 0;
    let mut direction_idx = 0;

    loop {
        if direction_idx == instructions.directions.len() {
            direction_idx = 0;
        }

        let direction = instructions.directions[direction_idx];

        if nodes.iter().all(|node| node.ends_with('Z')) {
            return steps;
        }

        for idx in 0..nodes.len() {
            let node = nodes[idx];
            let next_node = match direction {
                'R' => get_right(&node, &instructions).unwrap(),
                'L' => get_left(&node, &instructions).unwrap(),
                _ => panic!("Unknown direction: {}", direction),
            };

            nodes[idx] = next_node;
        }

        direction_idx += 1;
        steps += 1;
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"
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
