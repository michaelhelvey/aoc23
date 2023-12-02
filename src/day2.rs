use std::cmp::max;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{space1, u32};
use nom::combinator::value;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use rayon::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
}

/// Represents the presence of a single color in a grab from the bag.
#[derive(Debug, PartialEq)]
pub struct SingleColorGrab {
    color: Color,
    count: u32,
}

// A "game" represents a full game representing one or more grabs, each of which contain one or more
// different sets of colored die.
#[derive(Debug)]
pub struct Game {
    id: usize,
    grabs: Vec<Vec<SingleColorGrab>>,
}

/// Parses a single color grab from an input such as "3 blue" -> Grab { 3, Color::Blue }
fn parse_single_color_in_grab(input: &str) -> IResult<&str, SingleColorGrab> {
    let (input, (count, _, color)) = tuple((
        u32,
        space1,
        alt((
            value(Color::Blue, tag("blue")),
            value(Color::Green, tag("green")),
            value(Color::Red, tag("red")),
        )),
    ))(input)?;

    Ok((input, SingleColorGrab { count, color }))
}

fn parse_all_colors_for_grab(input: &str) -> IResult<&str, Vec<SingleColorGrab>> {
    separated_list1(tag(", "), parse_single_color_in_grab)(input)
}

impl<'a> TryFrom<&'a str> for Game {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    /// Parses a string into a Game
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let (input, (_, game_id, _)) = tuple((tag("Game "), u32, tag(": ")))(input)?;

        let (_, grabs) = separated_list1(tag("; "), parse_all_colors_for_grab)(input)?;

        let game = Self {
            id: game_id as usize,
            grabs,
        };

        Ok(game)
    }
}

const MAX_REDS: u32 = 12;
const MAX_GREENS: u32 = 13;
const MAX_BLUES: u32 = 14;

/// Iterates over all the games in the input and sums the IDS of the games where the number of cubes
/// that the elf pulls out of the max are possible given some max values.
pub fn find_possible_games(input: &str) -> usize {
    // note: this is _not_ the fastest way to do this lol...we could definitely just regex for big
    // numbers and probably be fine.  But I want to have FUN and use a PARSER COMBINATOR library and
    // what not goddammit!
    input
        .par_lines()
        .map(|line: &str| {
            let game: Game = line
                .try_into()
                .expect("Expected every line in the input file to be a valid game");

            for multi_grab in game.grabs {
                for single_grab in multi_grab {
                    match single_grab.color {
                        Color::Blue => {
                            if single_grab.count > MAX_BLUES {
                                return 0;
                            }
                        }
                        Color::Red => {
                            if single_grab.count > MAX_REDS {
                                return 0;
                            }
                        }
                        Color::Green => {
                            if single_grab.count > MAX_GREENS {
                                return 0;
                            }
                        }
                    }
                }
            }

            return game.id;
        })
        .sum()
}

/// Iterates over all the games in the input and computes the total sum power of the minimum number
/// of red, green, and blue cubes required to make each game possible.
pub fn sum_of_powers_of_fewest_cubes(input: &str) -> u32 {
    input
        .par_lines()
        .map(|line: &str| {
            let game: Game = line
                .try_into()
                .expect("Expected each line of the input to be a valid Game");

            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;

            for multi_grab in game.grabs {
                for single_grab in multi_grab {
                    match single_grab.color {
                        Color::Red => {
                            min_red = max(min_red, single_grab.count);
                        }
                        Color::Blue => {
                            min_blue = max(min_blue, single_grab.count);
                        }
                        Color::Green => {
                            min_green = max(min_green, single_grab.count);
                        }
                    }
                }
            }

            min_red * min_green * min_blue
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input_part_1() {
        let example_input = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#;

        let possible_games_sum = find_possible_games(example_input.trim());

        assert_eq!(possible_games_sum, 8);
    }

    #[test]
    fn test_example_input_part_2() {
        let example_input = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#;

        let sum_of_powers = sum_of_powers_of_fewest_cubes(example_input.trim());
        assert_eq!(sum_of_powers, 2286);
    }

    #[test]
    fn test_parse_single_color_in_grab() {
        let input = "3 blue";
        let (_, color) = parse_single_color_in_grab(input).unwrap();
        assert_eq!(
            color,
            SingleColorGrab {
                count: 3,
                color: Color::Blue
            }
        );
    }

    #[test]
    fn test_parse_game() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game: Game = input.try_into().unwrap();

        assert_eq!(game.id, 1);
        assert_eq!(game.grabs.len(), 3);

        assert_eq!(
            game.grabs,
            vec![
                vec![
                    SingleColorGrab {
                        count: 3,
                        color: Color::Blue
                    },
                    SingleColorGrab {
                        count: 4,
                        color: Color::Red
                    }
                ],
                vec![
                    SingleColorGrab {
                        count: 1,
                        color: Color::Red
                    },
                    SingleColorGrab {
                        count: 2,
                        color: Color::Green
                    },
                    SingleColorGrab {
                        count: 6,
                        color: Color::Blue
                    }
                ],
                vec![SingleColorGrab {
                    count: 2,
                    color: Color::Green
                }]
            ]
        )
    }
}
