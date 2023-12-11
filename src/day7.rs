use nom::character::complete::{alphanumeric1, newline, space1, u64};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn part_1_from(value: &str) -> Self {
        let mut state: HashMap<char, u64> = HashMap::new();

        let mut chars: Vec<_> = value.chars().collect();
        chars.sort();

        for c in chars {
            state.entry(c).and_modify(|x| *x += 1).or_insert(1);
        }

        Self::hand_type_from_state(&state)
    }

    fn hand_type_from_state(state: &HashMap<char, u64>) -> Self {
        match state.keys().count() {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 => {
                let mut values: Vec<_> = state.values().collect();
                values.sort();
                let (first, second, third) = (
                    values.pop().unwrap(),
                    values.pop().unwrap(),
                    values.pop().unwrap(),
                );

                match (first, second, third) {
                    (3, 1, 1) => HandType::ThreeOfAKind,
                    (2, 2, 1) => HandType::TwoPair,
                    _ => panic!("Unexpected values in 3 {:?}", (first, second, third)),
                }
            }
            2 => {
                // This feels fucking awful lol
                let mut values: Vec<_> = state.values().collect();
                values.sort();
                let (first, second) = (values.pop().unwrap(), values.pop().unwrap());

                match (first, second) {
                    (4, 1) => HandType::FourOfAKind,
                    (3, 2) => HandType::FullHouse,
                    _ => panic!("Unexpected values in 2 {:?}", (first, second)),
                }
            }
            1 => HandType::FiveOfAKind,
            count => panic!(
                "Expected value to only be 5 characters long, but got {}",
                count
            ),
        }
    }

    fn part_2_from(value: &str) -> Self {
        let mut state: HashMap<char, u64> = HashMap::new();

        let mut chars: Vec<_> = value.chars().collect();
        chars.sort();

        for c in chars {
            state.entry(c).and_modify(|x| *x += 1).or_insert(1);
        }

        let joker_count = *state.get(&'J').unwrap_or(&0);

        if joker_count == 5 {
            return HandType::FiveOfAKind;
        }

        let mut max_non_joker: Option<char> = None;
        let mut mnji = 0;

        for (c, count) in state.iter() {
            if *count > mnji && *c != 'J' {
                max_non_joker = Some(*c);
                mnji = *count;
            }
        }

        if let Some(max_non_joker) = max_non_joker {
            state.entry(max_non_joker).and_modify(|x| *x += joker_count);
        }

        if joker_count > 0 {
            state.remove(&'J');
        }

        Self::hand_type_from_state(&state)
    }
}

#[derive(Debug)]
struct GameEntry<'a> {
    hand: &'a str,
    bid: u64,
    typ: HandType,
}

fn parse_hand_line(input: &str) -> IResult<&str, GameEntry<'_>> {
    let (input, (hand, bid)) = separated_pair(alphanumeric1, space1, u64)(input)?;

    Ok((
        input,
        GameEntry {
            hand,
            bid,
            typ: HandType::part_1_from(hand),
        },
    ))
}

fn parse_hand_line_part2(input: &str) -> IResult<&str, GameEntry<'_>> {
    let (input, (hand, bid)) = separated_pair(alphanumeric1, space1, u64)(input)?;

    Ok((
        input,
        GameEntry {
            hand,
            bid,
            typ: HandType::part_2_from(hand),
        },
    ))
}

fn parse_input(input: &str) -> IResult<&str, Vec<GameEntry<'_>>> {
    separated_list1(newline, parse_hand_line)(input)
}

fn parse_input_part2(input: &str) -> IResult<&str, Vec<GameEntry<'_>>> {
    separated_list1(newline, parse_hand_line_part2)(input)
}

fn get_winnings(entries: &mut [GameEntry<'_>], scores: &HashMap<char, u64>) -> u64 {
    entries.sort_by(|a, b| {
        match (a.typ as usize).cmp(&(b.typ as usize)) {
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => {
                // if they are equal, then the "secondary" sorting rule applies where we have to walk
                // down the characters
                let mut idx = 0;

                loop {
                    if idx == 5 {
                        panic!("lol");
                    }

                    let a_char = a.hand.chars().nth(idx).unwrap();
                    let b_char = b.hand.chars().nth(idx).unwrap();

                    let ordering = scores
                        .get(&a_char)
                        .unwrap()
                        .cmp(scores.get(&b_char).unwrap());

                    match ordering {
                        std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                        std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                        std::cmp::Ordering::Equal => {
                            idx += 1;
                        }
                    }
                }
            }
        }
    });

    let mut result: u64 = 0;
    for (idx, entry) in entries.iter().enumerate() {
        let rank = idx + 1;
        result += entry.bid * rank as u64;
    }

    result
}

pub fn solve_part_1(input: &str) -> u64 {
    let (_, mut entries) = parse_input(input.trim()).unwrap();
    let mut scores: HashMap<char, u64> = HashMap::new();

    scores.insert('A', 13);
    scores.insert('K', 12);
    scores.insert('Q', 11);
    scores.insert('J', 10);
    scores.insert('T', 9);
    scores.insert('9', 8);
    scores.insert('8', 7);
    scores.insert('7', 6);
    scores.insert('6', 5);
    scores.insert('5', 4);
    scores.insert('4', 3);
    scores.insert('3', 2);
    scores.insert('2', 1);

    get_winnings(&mut entries, &scores)
}

pub fn solve_part_2(input: &str) -> u64 {
    let (_, mut entries) = parse_input_part2(input.trim()).unwrap();
    let mut scores: HashMap<char, u64> = HashMap::new();
    scores.insert('A', 13);
    scores.insert('K', 12);
    scores.insert('Q', 11);
    scores.insert('T', 10);
    scores.insert('9', 9);
    scores.insert('8', 8);
    scores.insert('7', 7);
    scores.insert('6', 6);
    scores.insert('5', 5);
    scores.insert('4', 4);
    scores.insert('3', 3);
    scores.insert('2', 2);
    scores.insert('J', 1);

    get_winnings(&mut entries, &scores)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
    "#;

    #[test]
    fn test_parse_entries() {
        let (_, entries) = parse_input(INPUT.trim()).unwrap();

        let first_entry = entries.first().unwrap();
        assert_eq!(first_entry.bid, 765);
        assert_eq!(first_entry.hand, "32T3K");
        assert_eq!(first_entry.typ, HandType::OnePair);

        let last_entry = entries.last().unwrap();
        assert_eq!(last_entry.bid, 483);
        assert_eq!(last_entry.hand, "QQQJA");
        assert_eq!(last_entry.typ, HandType::ThreeOfAKind)
    }

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(INPUT.trim()), 6440);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(INPUT.trim()), 5905);
    }
}
