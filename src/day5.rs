use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, not_line_ending, space1, u64},
    multi::{fold_many1, many0, separated_list1},
    sequence::{pair, tuple},
    IResult,
};

// e.g. "seed, soil" represents a single entry in the almanac
type InputMapKey<'a> = (&'a str, &'a str);
// (dest, source, range length)
type AlmanacRange = (u64, u64, u64);

// Represents the initial input in a very basic parsed format
#[derive(Debug, Default)]
struct Input<'a> {
    seeds: Vec<u64>,
    maps: HashMap<InputMapKey<'a>, Vec<AlmanacRange>>,
    // ptr_into_maps, start, end
    sorted_source_ranges: HashMap<InputMapKey<'a>, Vec<(u64, u64, u64)>>,
    // ptr_into_maps, start, end
    sorted_dest_ranges: HashMap<InputMapKey<'a>, Vec<(u64, u64, u64)>>,
    // no hash map because there's only one set of seed ranges, not one per element permutation
    sorted_seed_ranges: Vec<(u64, u64, u64)>,
}

const ELEMENT_ORDER: [&'static str; 8] = [
    "seed",
    "soil",
    "fertilizer",
    "water",
    "light",
    "temperature",
    "humidity",
    "location",
];

impl<'a> Input<'a> {
    // Build indexes of sorted ranges we can do binary search on later to make each map lookup at
    // least O(logN).
    fn pre_process(&mut self) {
        for (key, ranges) in &self.maps {
            let mut source_ranges: Vec<(u64, u64, u64)> = Vec::new();
            let mut dest_ranges: Vec<(u64, u64, u64)> = Vec::new();

            for (idx, (dest, source, range)) in ranges.iter().enumerate() {
                // e.g. 5, 2 = range of 5, 6 = 5, (5 + 2 - 1)
                let source_range = ((idx as u64), *source, (source + range - 1));
                source_ranges.push(source_range);

                let dest_range = ((idx as u64), *dest, (dest + range - 1));
                dest_ranges.push(dest_range);
            }

            source_ranges.sort_by(
                |(_, first_start, first_end), (_, second_start, second_end)| {
                    (first_start, first_end).cmp(&(&second_start, &second_end))
                },
            );

            dest_ranges.sort_by(
                |(_, first_start, first_end), (_, second_start, second_end)| {
                    (first_start, first_end).cmp(&(&second_start, &second_end))
                },
            );

            self.sorted_source_ranges.insert(*key, source_ranges);
            self.sorted_dest_ranges.insert(*key, dest_ranges);
        }

        let mut start_index = 0;
        let mut range_index = 1;
        while range_index < self.seeds.len() {
            // Fuck off Rust I just checked the length right there
            unsafe {
                let start = self.seeds.get_unchecked(start_index);
                let range = self.seeds.get_unchecked(range_index);

                self.sorted_seed_ranges
                    .push((start_index as u64, *start, start + range - 1))
            }

            start_index += 2;
            range_index += 2;
        }

        self.sorted_seed_ranges.sort_by(
            |(_, first_start, first_end), (_, second_start, second_end)| {
                (first_start, first_end).cmp(&(&second_start, &second_end))
            },
        );
    }

    // me write binary search, me know algorithms good
    fn search_for_idx_in_sorted_ranges(
        &self,
        sorted_ranges: &Vec<(u64, u64, u64)>,
        value: u64,
    ) -> Option<u64> {
        let mut low = 0;
        let mut high = sorted_ranges.len() - 1;

        while low <= high {
            let mid = (low + (high + 1)) / 2;

            // is value in range?
            let (idx, start, end) = sorted_ranges.get(mid).unwrap();
            if value >= *start && value <= *end {
                return Some(*idx);
            }

            if low == 0 && high == 0 {
                return None;
            }

            // is value on the right side? ignore left
            if value > *end {
                low = mid + 1;
            } else {
                // else if value is on the left, ignore right
                high = mid - 1;
            }
        }

        // if we get here and we haven't found a value in range, then one doesn't exixt
        None
    }

    // e.g. ("seed", "soil"), 50 = "give me the corresponding soil value for the seed value of 50"
    fn get_corresponding_for_ranges(
        &self,
        ranges: &Vec<(u64, u64, u64)>,
        sorted_ranges: &Vec<(u64, u64, u64)>,
        value: u64,
        reverse: bool,
    ) -> u64 {
        // The question that we have to answer is: is {value} in a given range, and if so which one?
        //  yes -> result = value - ({source} - {dest})
        //  no -> result = value
        // ...which leads us logically to the next question, which is how can we determine which (if
        // any) range {value} is in?  Binary search...
        let value_index_in_ranges = self.search_for_idx_in_sorted_ranges(&sorted_ranges, value);

        // Now we can trivially apply our formula above
        let result = match value_index_in_ranges {
            Some(range_index) => {
                let (dest, source, _) = ranges.get(range_index as usize).unwrap();
                // These conversions are...pretty stupid!  TL;DR the delta is source - dest if value
                // corresponds to a source, but it's dest - source if value corresponds to a dest,
                // and we have to handle both so that we can traverse backwards up the tree to get
                // part 2's answer
                match reverse {
                    false => {
                        let delta = (*source as i64) - (*dest as i64);
                        ((value as i64) - delta) as u64
                    }
                    true => {
                        let delta = (*dest as i64) - (*source as i64);
                        ((value as i64) - delta) as u64
                    }
                }
            }
            None => value,
        };

        result
    }

    // e.g. ("seed", "soil"), 50 = "give me the corresponding soil value for the seed value of 50"
    fn get_corresponding(&self, key: InputMapKey<'a>, value: u64) -> u64 {
        let ranges = self.maps.get(&key).unwrap();
        let sorted_ranges = self.sorted_source_ranges.get(&key).unwrap();

        self.get_corresponding_for_ranges(&ranges, &sorted_ranges, value, false)
    }

    // e.g. "given a location, find me the humidity" (where location =  50). key order is the same
    // as get_corresponding so even though it's in reverse we still query by ("humidity, location"),
    // 50 in this example
    fn get_corresponding_reverse(&self, key: InputMapKey<'a>, value: u64) -> u64 {
        let ranges = self.maps.get(&key).unwrap();
        let sorted_ranges = self.sorted_dest_ranges.get(&key).unwrap();

        self.get_corresponding_for_ranges(&ranges, &sorted_ranges, value, true)
    }

    fn get_location_for_seed(&self, seed: u64) -> u64 {
        let mut ptr_1 = 0;
        let mut ptr_2 = 1;
        let mut value: u64 = seed;

        while ptr_2 < ELEMENT_ORDER.len() {
            let key = {
                (*unsafe { ELEMENT_ORDER.get_unchecked(ptr_1) }, *unsafe {
                    ELEMENT_ORDER.get_unchecked(ptr_2)
                })
            };

            value = self.get_corresponding(key, value);
            ptr_1 += 1;
            ptr_2 += 1;
        }

        value
    }

    fn get_maybe_seed_for_location(&self, location: u64) -> u64 {
        let mut ptr_1 = ELEMENT_ORDER.len() - 2;
        let mut ptr_2 = ELEMENT_ORDER.len() - 1;
        let mut value: u64 = location;

        while ptr_2 >= 1 {
            let key = {
                (*unsafe { ELEMENT_ORDER.get_unchecked(ptr_1) }, *unsafe {
                    ELEMENT_ORDER.get_unchecked(ptr_2)
                })
            };

            value = self.get_corresponding_reverse(key, value);

            // I'm literally too lazy
            match ptr_1 {
                0 => break,
                _ => {
                    ptr_1 -= 1;
                    ptr_2 -= 1;
                }
            }
        }

        // if there is a seed value for this location, this would be it
        value
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
    let (input, (num1, _, num2, _, num3)) = tuple((u64, space1, u64, space1, u64))(input)?;

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
    let (input, (_, seeds)) = tuple((tag("seeds: "), separated_list1(space1, u64)))(input)?;

    let (_, maps) = fold_many1(
        pair(map_parser, many0(newline)),
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
            sorted_dest_ranges: HashMap::new(),
            sorted_seed_ranges: Vec::new(),
        },
    ))
}

pub fn find_lowest_location_number(input: &str) -> u64 {
    let (_, mut processor) = parse_input(input.trim()).expect("Expected to get valid input");
    processor.pre_process();

    let mut value: u64 = u64::MAX;
    for seed in &processor.seeds {
        value = std::cmp::min(value, processor.get_location_for_seed(*seed));
    }

    value
}

pub fn find_lowest_location_for_seed_ranges(input: &str) -> u64 {
    let (_, mut processor) = parse_input(input.trim()).expect("Expected to get valid input");
    processor.pre_process();

    let mut locations = processor
        .maps
        .get(&("humidity", "location"))
        .unwrap()
        .iter()
        .map(|(dest, _, range)| (*dest, *dest + range - 1))
        .collect::<Vec<(u64, u64)>>();

    locations.sort();

    let mut result: Option<u64> = None;

    // This is a great idea if this was actually a valid list of locations...somehow I have to
    // figure out a (small) list of valid locations to check.
    for (start, end) in locations {
        for x in start..=end {
            let location = x;
            let maybe_seed_value = processor.get_maybe_seed_for_location(location);
            if let Some(_) = processor
                .search_for_idx_in_sorted_ranges(&processor.sorted_seed_ranges, maybe_seed_value)
            {
                println!("seed {}, location {}", maybe_seed_value, location);
                result = Some(location);
                break;
            }
        }
    }

    result.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_INPUT: &'static str = r#"
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

        let first_line = result
            .maps
            .get(&("soil", "fertilizer"))
            .unwrap()
            .get(0)
            .unwrap();
        assert_eq!(*first_line, (0, 15, 37));
    }

    #[test]
    fn test_get_corresponding() {
        let (_, mut processor) = parse_input(EXAMPLE_INPUT.trim()).unwrap();
        processor.pre_process();

        let soil_for_seed = processor.get_corresponding(("seed", "soil"), 79);
        assert_eq!(soil_for_seed, 81);

        let soil_for_seed = processor.get_corresponding(("seed", "soil"), 14);
        assert_eq!(soil_for_seed, 14);

        let soil_for_seed = processor.get_corresponding(("seed", "soil"), 55);
        assert_eq!(soil_for_seed, 57);

        let fertilizer_for_soil = processor.get_corresponding(("soil", "fertilizer"), 14);
        assert_eq!(fertilizer_for_soil, 53);
    }

    #[test]
    fn test_get_corresponding_reverse() {
        let (_, mut processor) = parse_input(EXAMPLE_INPUT.trim()).unwrap();
        processor.pre_process();

        let humidity_for_location =
            processor.get_corresponding_reverse(("humidity", "location"), 46);
        assert_eq!(humidity_for_location, 46);

        let temperature_for_humidity =
            processor.get_corresponding_reverse(("temperature", "humidity"), 46);
        assert_eq!(temperature_for_humidity, 45);

        let light_for_temperature =
            processor.get_corresponding_reverse(("light", "temperature"), 45);
        assert_eq!(light_for_temperature, 77);
    }

    #[test]
    fn test_day_5_part_1() {
        assert_eq!(find_lowest_location_number(EXAMPLE_INPUT), 35);
    }

    #[test]
    fn test_day_5_part_2() {
        assert_eq!(find_lowest_location_for_seed_ranges(EXAMPLE_INPUT), 46);
    }
}
