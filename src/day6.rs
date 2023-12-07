use nom::bytes::complete::{tag, take};
use nom::character::complete::{newline, space0, space1, u64};
use nom::combinator::map_res;
use nom::multi::{many1, separated_list1};
use nom::sequence::{terminated, tuple};
use nom::IResult;

fn parse_input(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    let (input, _) = tuple((tag("Time:"), space1))(input)?;
    let (input, times) = separated_list1(space1, u64)(input)?;
    let (input, _) = tuple((newline, tag("Distance:"), space1))(input)?;
    let (input, distances) = separated_list1(space1, u64)(input)?;
    let times_distances: Vec<(u64, u64)> = times
        .iter()
        .map(|x| *x)
        .zip(distances.iter().map(|x| *x))
        .collect();

    Ok((input, times_distances))
}

/// Parses a single integer from the input, e.g. given "123" will return "1" as a u64.
fn parse_single_integer(input: &str) -> IResult<&str, u64> {
    let (input, digit) = map_res(take(1 as usize), |c: &str| c.parse::<u64>())(input)?;
    Ok((input, digit))
}

fn parse_input_part_2(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    let (input, _) = tuple((tag("Time:"), space1))(input)?;
    let (input, times) = many1(terminated(parse_single_integer, space0))(input)?;
    let (input, _) = tuple((newline, tag("Distance:"), space1))(input)?;
    let (input, distances) = many1(terminated(parse_single_integer, space0))(input)?;

    let time = times.iter().fold(0, |acc, x| (acc * 10) + *x);
    let distance = distances.iter().fold(0, |acc, x| (acc * 10) + *x);

    Ok((input, vec![(time, distance)]))
}

fn get_num_ways_to_solve(races: &Vec<(u64, u64)>) -> u64 {
    let mut final_result: u64 = 1;

    for (time, record_distance) in races.iter() {
        // its inefficient to check every hold/speed combination so we can just use two pointers
        // from each end until each "beats" the record and then the difference is the number of ways
        // we can beat the record
        let mut hold_lower_bound: u64 = 1;
        let mut hold_upper_bound: u64 = time - 1;

        // walk up from the beginning
        while hold_lower_bound < *time {
            // travel_time = time - hold
            // distance_traveled = hold * travel_time
            let travel_time = time - hold_lower_bound;
            let distance_traveled = hold_lower_bound * travel_time;

            if distance_traveled > *record_distance {
                break;
            }

            hold_lower_bound += 1;
            // I'm assuming that a solution exists for each scenario so I'm purposely ignoring
            // the case where we don't get to a record
        }

        while hold_upper_bound > 0 {
            let travel_time = time - hold_upper_bound;
            let distance_traveled = hold_upper_bound * travel_time;

            if distance_traveled > *record_distance {
                break;
            }

            hold_upper_bound -= 1;
        }

        let num_ways_to_beat_record = (hold_upper_bound + 1) - hold_lower_bound;
        final_result *= num_ways_to_beat_record;
    }

    final_result
}

pub fn solve_part_1(input: &str) -> u64 {
    let (_, times_distances) = parse_input(input.trim()).unwrap();
    get_num_ways_to_solve(&times_distances)
}

pub fn solve_part_2(input: &str) -> u64 {
    let (_, times_distances) = parse_input_part_2(input.trim()).unwrap();
    get_num_ways_to_solve(&times_distances)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"
Time:      7  15   30
Distance:  9  40  200
        "#;

    #[test]
    fn test_part_1() {
        let result = solve_part_1(INPUT.trim());
        assert_eq!(result, 288);
    }

    #[test]
    fn test_part_2_parse() {
        let (_, part_2_parsed) = parse_input_part_2(INPUT.trim()).unwrap();

        let (time, distance) = part_2_parsed.first().unwrap();
        assert_eq!(*time, 71530);
        assert_eq!(*distance, 940200);
    }

    #[test]
    fn test_part_2() {
        let result = solve_part_2(INPUT.trim());
        assert_eq!(result, 71503);
    }
}
