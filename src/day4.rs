use std::collections::{HashMap, HashSet};

use nom::bytes::complete::tag;
use nom::character::complete::{space0, space1, u32};
use nom::multi::many1;
use nom::sequence::{delimited, tuple};
use nom::IResult;

fn parse_card_line(input: &str) -> IResult<&str, (u32, Vec<u32>, Vec<u32>)> {
    let (input, (_, _, card_id, _, _)) =
        tuple((tag("Card"), space1, u32, tag(":"), space1))(input)?;

    let (input, (winning, _, has)) = tuple((
        many1(delimited(space0, u32, space0)),
        delimited(space0, tag("|"), space0),
        many1(delimited(space0, u32, space0)),
    ))(input)?;

    Ok((input, (card_id, winning, has)))
}

fn count_winning_numbers(winning: Vec<u32>, has: Vec<u32>) -> u32 {
    let mut winning_set: HashSet<u32> = HashSet::new();
    for score in winning {
        winning_set.insert(score);
    }

    let mut x = 0;
    for score in has {
        if winning_set.contains(&score) {
            x += 1;
        }
    }

    x
}

fn win_card(
    card_id: u32,
    current_index: usize,
    win_count: u32,
    cards: &Vec<(u32, u32)>,
    store: &mut HashMap<u32, u32>,
    _depth: u32,
) {
    // Increment the number of wins for that card
    store
        .entry(card_id)
        .and_modify(|wins| {
            *wins += 1;
        })
        .or_insert(1);

    for i in (current_index + 1)..(current_index + (win_count as usize) + 1) {
        let (next_card_id, next_win_count) = cards.get(i).expect(
            "AOC promised me that it will never make me copy a card past the end of the table",
        );

        win_card(*next_card_id, i, *next_win_count, cards, store, _depth + 1);
    }
}

pub fn sum_recursive_won_scratchcards(input: &str) -> u32 {
    // Maps from a card ID to how many times it's been won
    let mut store: HashMap<u32, u32> = HashMap::new();

    let cards: Vec<(u32, u32)> = input
        .lines()
        .map(|line: &str| {
            let (_, (card_id, winning, has)) = parse_card_line(line).unwrap();
            let win_count = count_winning_numbers(winning, has);

            (card_id, win_count)
        })
        .collect();

    for (idx, (card_id, win_count)) in cards.iter().enumerate() {
        win_card(*card_id, idx, *win_count, &cards, &mut store, 0);
    }

    store.values().sum()
}

pub fn sum_winning_scores(input: &str) -> u32 {
    input
        .lines()
        .map(|line: &str| {
            let (_, (_, winning, has)) = parse_card_line(line).unwrap();

            let win_count = count_winning_numbers(winning, has);

            match win_count {
                0 => 0,
                _ => 1 << (win_count - 1),
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_winning_scores() {
        let input = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "#
        .trim();

        assert_eq!(sum_winning_scores(input), 13);
    }

    #[test]
    fn test_find_recursive_sum() {
        let input = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
        "#
        .trim();

        assert_eq!(sum_recursive_won_scratchcards(input), 30);
    }
}
