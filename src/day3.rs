use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy)]
pub struct NumberToken {
    id: u32, // this is what happens when you don't refactor for part 2 and just brute force it :(
    start: Location,
    end: Location,
    value: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct SymbolToken {
    location: Location,
    value: char,
}

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct Location {
    column: usize,
    row: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum Token {
    Number(NumberToken),
    Symbol(SymbolToken),
}

// This is arguably kind of sloppy, but it's an easy way to ask "do these two tokens have the same
// value" regardless of location.
impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0.value == r0.value,
            (Self::Symbol(l0), Self::Symbol(r0)) => l0.value == r0.value,
            _ => false,
        }
    }
}

impl From<(usize, usize)> for Location {
    fn from(value: (usize, usize)) -> Self {
        Location {
            row: value.0,
            column: value.1,
        }
    }
}

#[derive(Debug, Default)]
pub struct Row {
    // Sparse array mapping X coordinates to a vertex
    nodes: HashMap<usize, Token>,
}

/// Represents a logically infinite series of coordinates from (0, 0) as the top left corner to
/// (Inf, Inf) as the bottom right corner.
#[derive(Debug)]
pub struct Grid {
    rows: Vec<Row>,
}

impl Grid {
    pub fn new() -> Self {
        Self { rows: Vec::new() }
    }

    pub fn from_input(input: &str) -> Self {
        let mut grid = Grid::new();
        let mut token_id = 0;
        for (row, line) in input.lines().enumerate() {
            let tokens = tokenize_line(row, &mut token_id, line);

            for token in tokens {
                match token {
                    Token::Number(number) => {
                        for i in number.start.column..number.end.column + 1 {
                            grid.insert((number.start.row, i).into(), token)
                        }
                    }
                    Token::Symbol(symbol) => grid.insert(symbol.location, token),
                }
            }
        }

        grid
    }

    pub fn get(&self, coord: Location) -> Option<&Token> {
        // (0, 0) represents the first character in the input, so the top left character when viewed
        // as a grid
        let row = coord.row;
        let col = coord.column;

        let row = self.rows.get(row);

        match row {
            Some(row) => row.nodes.get(&col),
            None => None,
        }
    }

    pub fn insert(&mut self, coord: Location, token: Token) {
        let row = coord.row;
        let col = coord.column;

        while self.rows.get(row).is_none() {
            self.rows.push(Row::default())
        }

        // SAFETY: we just created the row in the line above
        let row = unsafe { self.rows.get_unchecked_mut(row) };
        row.nodes.insert(col, token);
    }

    // Let it be known I'm sure there's a better way to do this but my child is going to wake up
    // from her nap soon :(
    pub fn has_adjacent_symbol(&self, start: Location, end: Location) -> bool {
        let mut row = match start.row {
            0 => start.row,
            _ => start.row - 1,
        };

        let start_col = match start.column {
            0 => start.column,
            _ => start.column - 1,
        };

        while row < start.row + 2 {
            let mut col = start_col;

            while col < end.column + 2 {
                match self.get((row, col).into()) {
                    Some(Token::Symbol(_)) => {
                        return true;
                    }
                    _ => (),
                }

                col += 1;
            }

            row += 1;
        }

        false
    }

    /// Blatant copy of "has_adjacent_symbol" but with some extra logic...not DRY but I'm rushing here :)
    pub fn gear_ratio(&self, loc: Location) -> u32 {
        let mut row = match loc.row {
            0 => loc.row,
            _ => loc.row - 1,
        };

        let start_col = match loc.column {
            0 => loc.column,
            _ => loc.column - 1,
        };

        let mut found_numbers: Vec<u32> = vec![];
        let mut seen: HashSet<u32> = HashSet::new();

        while row < loc.row + 2 {
            let mut col = start_col;

            while col < loc.column + 2 {
                let token = self.get((row, col).into());
                match token {
                    Some(Token::Number(number)) => {
                        if !seen.contains(&number.id) {
                            found_numbers.push(number.value);
                            seen.insert(number.id);
                        }

                        if found_numbers.len() > 2 {
                            return 0; // fast path
                        }
                    }
                    _ => (),
                }

                col += 1;
            }

            row += 1;
        }

        if found_numbers.len() == 2 {
            let first = found_numbers.get(0).unwrap();
            let second = found_numbers.get(1).unwrap();

            return first * second;
        }

        0
    }

    pub fn sum_of_gear_ratios(&self) -> u32 {
        let mut sum = 0;
        for row in &self.rows {
            for token in row.nodes.values() {
                if let Token::Symbol(SymbolToken {
                    value: '*',
                    location,
                }) = token
                {
                    let ratio = self.gear_ratio(*location);
                    sum += ratio;
                }
            }
        }

        sum
    }

    pub fn sum_numbers_with_adjencent_symbols(&self) -> u32 {
        let mut sum = 0;

        let mut seen: HashSet<u32> = HashSet::new();

        for row in &self.rows {
            for token in row.nodes.values() {
                if let Token::Number(number) = token {
                    // Find the "adjancent box" around the number and check for symbols.
                    if !seen.contains(&number.id)
                        && self.has_adjacent_symbol(number.start, number.end)
                    {
                        sum += number.value;
                        seen.insert(number.id);
                    }
                }
            }
        }

        sum
    }
}

/// Given a string like "123...456..." returns vec![123, 456] (as Tokens, which contain their
/// start and end information as metadata)
fn tokenize_line(row: usize, num_token_id: &mut u32, line: &str) -> Vec<Token> {
    let mut char_iter = line.chars().peekable();

    let mut tokens: Vec<Token> = Vec::new();

    let mut col: usize = 0;
    while let Some(c) = char_iter.next() {
        if c == '.' {
            col += 1;
            continue;
        }

        if let Some(current_digit) = c.to_digit(10) {
            let mut current_value: u32 = 0;
            let start_coord: Location = (row, col).into();
            let mut end_coord: Location = start_coord.clone();

            current_value = (current_value * 10) + current_digit;

            while let Some(next_digit) = char_iter.peek().and_then(|c| c.to_digit(10)) {
                current_value = (current_value * 10) + next_digit;
                _ = char_iter.next();

                col += 1;
                end_coord.column += 1;
            }

            tokens.push(Token::Number(NumberToken {
                id: *num_token_id,
                start: start_coord,
                end: end_coord,
                value: current_value,
            }));

            *num_token_id += 1;
            col += 1;
            continue;
        }

        // For the purposes of this problem input, we can consider everything that's not a '.' or a
        // number to be a "symbol"
        tokens.push(Token::Symbol(SymbolToken {
            location: (row, col).into(),
            value: c,
        }));
        col += 1;
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Token {
        fn new_from_num(value: u32) -> Self {
            Token::Number(NumberToken {
                id: 0,
                start: Location::default(),
                end: Location::default(),
                value,
            })
        }

        fn new_from_symbol(value: char) -> Self {
            Token::Symbol(SymbolToken {
                location: Location::default(),
                value,
            })
        }
    }

    #[test]
    fn test_parse_numbers_from_line() {
        let input = "467..114..$.";

        assert_eq!(
            tokenize_line(123, &mut 0, input),
            vec![
                Token::new_from_num(467),
                Token::new_from_num(114),
                Token::new_from_symbol('$')
            ]
        );
    }

    #[test]
    fn test_grid_semantics() {
        let mut grid = Grid::new();
        assert!(grid.get((4, 5).into()).is_none());

        // The grid is logically infinite so you can insert at any coordinate safely:
        let number_token = Token::new_from_num(69);
        grid.insert((4, 5).into(), number_token);

        assert_eq!(*grid.get((4, 5).into()).unwrap(), number_token);
    }

    #[test]
    fn test_has_adjancent_symbol() {
        let input = "467";
        let grid = Grid::from_input(input);

        let initial_token = grid.rows.get(0).unwrap().nodes.get(&0).unwrap();
        assert!(matches!(initial_token, Token::Number(..)));

        let input = "467$";
        let grid = Grid::from_input(input);
        assert!(grid.has_adjacent_symbol((0, 0).into(), (0, 2).into()));

        let input = r#"
...123...
...467...
......$..
......123
.........
.....123.
....@....
        "#
        .trim();

        let grid = Grid::from_input(input);
        assert!(grid.has_adjacent_symbol((1, 3).into(), (1, 5).into()));
        assert_eq!(
            grid.has_adjacent_symbol((0, 3).into(), (0, 5).into()),
            false
        );
        assert!(grid.has_adjacent_symbol((3, 6).into(), (3, 8).into()));
        assert!(grid.has_adjacent_symbol((5, 5).into(), (5, 7).into()));
    }

    #[test]
    fn test_has_adjancent_symbol2() {
        let input = r#"
......755.
...$.*....
        "#
        .trim();

        let grid = Grid::from_input(input);
        assert!(grid.has_adjacent_symbol((1, 6).into(), (1, 8).into()));
    }

    #[test]
    fn test_parse_into_grid() {
        let input = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
        "#
        .trim();

        let grid = Grid::from_input(input);
        assert_eq!(grid.sum_numbers_with_adjencent_symbols(), 4361);
    }
    #[test]
    fn test_parse_gear_ratios() {
        let input = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
        "#
        .trim();

        let grid = Grid::from_input(input);
        assert_eq!(grid.sum_of_gear_ratios(), 467835);
    }
}
