use color_eyre::Result;
use rayon::prelude::*;

fn main() -> Result<()> {
    color_eyre::install()?;
    let input = include_str!("../../input/day_1.txt").trim();

    println!("sum of part 1: {}", get_puzzle_result(input, false));
    println!("sum of part 2: {}", get_puzzle_result(input, true));

    Ok(())
}

fn get_puzzle_result(input: &'static str, recogize_strs: bool) -> u32 {
    let sum = input
        .par_lines()
        .map(|line: &str| {
            if let Some((first, last)) = aoc::day1::find_first_last(line, recogize_strs) {
                return (first * 10) + last;
            }

            panic!("line {line} did not contain a number");
        })
        .sum();

    sum
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

        let result = get_puzzle_result(example_input.trim(), false);
        assert_eq!(result, 142);
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

        let result = get_puzzle_result(input.trim(), true);
        assert_eq!(result, 281);
    }
}
