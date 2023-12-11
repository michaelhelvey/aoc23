use nom::{
    character::complete::{i64, space0},
    multi::fold_many1,
    sequence::tuple,
    IResult,
};

fn parse_line_of_i64(input: &str) -> IResult<&str, Vec<i64>> {
    fold_many1(
        tuple((i64, space0)),
        Vec::new,
        |mut acc: Vec<i64>, (i, _)| {
            acc.push(i);
            acc
        },
    )(input)
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .trim()
        .lines()
        .map(|line| parse_line_of_i64(line.trim()).unwrap().1)
        .collect::<Vec<_>>()
}

/// Given an array like "0 3 6 9 12 15" reduce it to "0 0 0 0" and return the intermediate sequences
fn get_between_seqs(seq: &[i64]) -> Vec<Vec<i64>> {
    let mut current_seq = seq.to_vec();
    let mut between_seqs = Vec::<Vec<i64>>::new();

    loop {
        let mut next_seq = Vec::<i64>::new();
        let mut zeroes = 0;
        for (idx, num) in current_seq[1..].iter().enumerate() {
            let real_index = idx + 1;
            let previous = current_seq.get(real_index - 1).unwrap();
            let diff = num - previous;

            next_seq.push(diff);

            if diff == 0 {
                zeroes += 1;
            }
        }

        if zeroes == next_seq.len() {
            break;
        }

        between_seqs.push(next_seq.clone());
        current_seq = next_seq;
    }

    between_seqs
}

pub fn solve_part_1(input: &str) -> i64 {
    parse_input(input).iter().fold(0, |result, top_seq| {
        let between_seqs = get_between_seqs(top_seq);

        let below = between_seqs.iter().rev().fold(0, |below, seq| {
            let left = seq.last().unwrap();
            below + left
        });
        let left = top_seq.last().unwrap();
        let next_value = left + below;

        result + next_value
    })
}

pub fn solve_part_2(input: &str) -> i64 {
    parse_input(input).iter().fold(0, |result, top_seq| {
        let between_seqs = get_between_seqs(top_seq);

        let below = between_seqs.iter().rev().fold(0, |below, seq| {
            let right = seq.first().unwrap();
            right - below
        });

        let right = top_seq.first().unwrap();
        let next_value = right - below;

        result + next_value
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let input = "1 -2 4";
        assert_eq!(parse_line_of_i64(input), Ok(("", vec![1, -2, 4])));
    }

    #[test]
    fn test_example_input() {
        let input = r#"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
        "#;

        assert_eq!(solve_part_1(input.trim()), 114);
    }

    #[test]
    fn test_example_part_2() {
        let input = r#"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
        "#;

        assert_eq!(solve_part_2(input.trim()), 2);
    }
}
