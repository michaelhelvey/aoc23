use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, not_line_ending, space1, u32},
    multi::{fold_many1, separated_list1},
    sequence::{pair, tuple},
    IResult,
};

// e.g. "seed, soil" represents a single entry in the almanac
type InputMapKey<'a> = (&'a str, &'a str);
// (dest, source, range length)
type AlmanacRange = (u32, u32, u32);

// Represents the initial input in a very basic parsed format
#[derive(Debug, Default)]
struct Input<'a> {
    seeds: Vec<u32>,
    maps: HashMap<InputMapKey<'a>, Vec<AlmanacRange>>,
    // ptr_into_maps, start, end
    sorted_source_ranges: HashMap<InputMapKey<'a>, Vec<(u32, u32, u32)>>,
}

impl<'a> Input<'a> {
    // Build indexes of sorted ranges we can do binary search on later to make each map lookup at
    // least O(logN).
    fn pre_process(&mut self) {
        for (key, ranges) in &self.maps {
            let mut source_ranges: Vec<(u32, u32, u32)> = Vec::new();
            for (idx, (_, source, range)) in ranges.iter().enumerate() {
                // e.g. 5, 2 = range of 5, 6 = 5, (5 + 2 - 1)
                let range = ((idx as u32), *source, (source + range - 1));
                source_ranges.push(range);
            }

            source_ranges.sort();
            self.sorted_source_ranges.insert(*key, source_ranges);
        }
    }

    // e.g. ("seed", "soil"), 50 = "give me the corresponding soil value for the seed value of 50"
    fn get_corresponding(&self, key: InputMapKey<'a>, value: u32) -> u32 {
        let ranges = self.maps.get(&key).unwrap();

        // question 1: is {value} in one of the source ranges?
        //  yes -> result = value - ({source} - {dest})
        //  no -> result = value

        // question 2: how can we quickly determine if {value} is in a source range, and if so,
        // which one?
        // it's very quick if we use binary search on sorted ranges (which we can have from
        // pre-indexing everything)...remember that

        0
    }
}

fn element_parser<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    alt((
        tag("seed"),
        tag("soil"),
        tag("fertilizer"),
        tag("water"),
        tag("light"),
        tag("temperature"),
        tag("humidity"),
        tag("location"),
    ))(input)
}

fn map_line_parser<'a>(input: &'a str) -> IResult<&'a str, InputMapKey<'a>> {
    let (input, (from, _, to, _)) =
        tuple((element_parser, tag("-to-"), element_parser, not_line_ending))(input)?;

    Ok((input, (from, to)))
}

fn map_num_line<'a>(input: &'a str) -> IResult<&'a str, AlmanacRange> {
    let (input, (num1, _, num2, _, num3)) = tuple((u32, space1, u32, space1, u32))(input)?;

    Ok((input, (num1, num2, num3)))
}

fn map_parser<'a>(input: &'a str) -> IResult<&'a str, (InputMapKey<'a>, Vec<AlmanacRange>)> {
    let (input, (map_line, _, num_list)) = tuple((
        map_line_parser,
        newline,
        separated_list1(newline, map_num_line),
    ))(input)?;

    Ok((input, (map_line, num_list)))
}

// I realize that this is the stupidest way to do this but listen, I wanted to come out of AOC this
// year knowing the nom parser combinator library better.
fn parse_input<'a>(input: &'a str) -> IResult<&'a str, Input<'a>> {
    let (input, (_, seeds)) = tuple((tag("seeds: "), separated_list1(space1, u32)))(input)?;

    let (_, maps) = fold_many1(
        pair(map_parser, newline),
        HashMap::new,
        |mut acc: HashMap<InputMapKey<'a>, Vec<AlmanacRange>>, (((from, to), map_list), _)| {
            acc.insert((from, to), map_list);
            acc
        },
    )(input.trim())?;

    Ok((
        input,
        Input {
            seeds,
            maps,
            sorted_source_ranges: HashMap::new(),
        },
    ))
}

pub fn find_lowest_location_number(input: &str) -> u32 {
    let parsed_input = parse_input(input.trim()).expect("Expected to get valid input");

    println!("parsed_input: {:?}", parsed_input);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_line_parser() {
        let input = "humidity-to-location map:\n";
        let (_, map_line) = map_line_parser(input).unwrap();
        assert_eq!(map_line, ("humidity", "location"));
    }

    #[test]
    fn test_map_parser() {
        let input = r#"
humidity-to-location map:
1 2 3
2 3 4
"#;
        let (_, ((from, to), vecs)) = map_parser(input.trim()).unwrap();

        assert_eq!(from, "humidity");
        assert_eq!(to, "location");
        assert_eq!(vecs, vec![(1, 2, 3), (2, 3, 4)]);
    }

    #[test]
    fn test_multiple_maps() {
        let input = r#"
seeds: 1 2 3

seed-to-soil map:
1 2 3
2 3 4

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15
        "#;

        let (_, result) = parse_input(input.trim()).unwrap();

        let first_line = result.maps.get(&("seed", "soil")).unwrap().get(0).unwrap();
        assert_eq!(*first_line, (1, 2, 3));
    }

    #[test]
    fn test_day_5() {
        let input = r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
        "#;

        find_lowest_location_number(input);
    }

    #[test]
    fn test_sort_tuples() {
        let mut tuples = vec![(5, 6), (1, 4), (6, 8), (5, 7), (2, 10)];
        tuples.sort();

        println!("tuples: {:?}", tuples);
        assert!(false);
    }
}
