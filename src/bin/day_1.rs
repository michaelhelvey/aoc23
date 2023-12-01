use color_eyre::Result;
use rayon::prelude::*;
use std::collections::HashMap;

fn tokenize_string(input: &str, str_num_map: Option<&HashMap<&str, u32>>) -> (u32, u32) {
    // puzzle contains only non-zero so we can cheat and not use option
    let mut first_num = 0;
    let mut last_num = 0;

    let mut iter = input.chars();
    let mut i = 0;

    while let Some(c) = iter.next() {
        if c.is_digit(10) {
            if first_num == 0 {
                first_num = c.to_digit(10).unwrap()
            } else {
                last_num = c.to_digit(10).unwrap()
            }

            i += 1;
            continue;
        }

        // This is ugly and imperative, I could definitely clean it up
        match str_num_map {
            Some(str_num_map) => {
                if input.len() >= i + 5 {
                    if let Some(five_match) = str_num_map.get(&input[i..i + 5]) {
                        if first_num == 0 {
                            first_num = *five_match
                        } else {
                            last_num = *five_match
                        }

                        i += 5;
                        _ = iter.nth(3);
                        continue;
                    }
                }

                if input.len() >= i + 4 {
                    if let Some(four_match) = str_num_map.get(&input[i..i + 4]) {
                        if first_num == 0 {
                            first_num = *four_match;
                        } else {
                            last_num = *four_match;
                        }
                        i += 4;
                        _ = iter.nth(2);
                        continue;
                    }
                }

                if input.len() >= i + 3 {
                    if let Some(three_match) = str_num_map.get(&input[i..i + 3]) {
                        if first_num == 0 {
                            first_num = *three_match;
                        } else {
                            last_num = *three_match;
                        }
                        i += 3;
                        _ = iter.nth(1);
                        continue;
                    }
                }
            }
            None => {}
        };

        i += 1;
    }

    if last_num == 0 {
        last_num = first_num;
    }

    (first_num, last_num)
}

fn get_puzzle_result(input: &str, str_num_map: Option<&HashMap<&'static str, u32>>) -> u32 {
    input
        .par_lines() // this...actually doesn't speed up the result at all, which is really surprising
        .map(|line: &str| {
            let (first, last) = tokenize_string(line, str_num_map);
            (first * 10) + last
        })
        .sum()
}

fn build_str_lookup_table() -> HashMap<&'static str, u32> {
    let mut str_num_map = HashMap::<&str, u32>::new();
    str_num_map.insert("one", 1);
    str_num_map.insert("two", 2);
    str_num_map.insert("three", 3);
    str_num_map.insert("four", 4);
    str_num_map.insert("five", 5);
    str_num_map.insert("six", 6);
    str_num_map.insert("seven", 7);
    str_num_map.insert("eight", 8);
    str_num_map.insert("nine", 9);

    str_num_map
}

fn main() -> Result<()> {
    let input = include_str!("../../input/day_1.txt").trim();

    let str_num_map = build_str_lookup_table();
    println!("sum of part 1: {}", get_puzzle_result(input, None));
    println!(
        "sum of part 2: {}",
        get_puzzle_result(input, Some(&str_num_map))
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example_input() {
        let example_input = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
        "#;

        let result = get_puzzle_result(example_input.trim(), None);
        assert_eq!(result, 142);
    }

    #[test]
    fn test_tokenizer() {
        let map = build_str_lookup_table();
        let result = tokenize_string("threeabcfourdefonetwothree123sixteen", Some(&map));
        assert_eq!(result, (3, 6));
    }

    #[test]
    fn test_tokenizer_2() {
        let map = build_str_lookup_table();
        let result = tokenize_string("threeabcfourdefonetwothree123sixteennine", Some(&map));
        assert_eq!(result, (3, 9));
    }

    #[test]
    fn test_tokenizer_3() {
        let map = build_str_lookup_table();
        let result = tokenize_string("5eightvsrzjmdbtqhhqtjfjrhllhbgzgzjzvdhddstxpp4", Some(&map));
        assert_eq!(result, (5, 4));
    }

    #[test]
    fn test_part_2_example() {
        let input = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
            "#;

        let map = build_str_lookup_table();
        let result = get_puzzle_result(input.trim(), Some(&map));
        assert_eq!(result, 281);
    }
}
