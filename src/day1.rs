// We once again skip Rust's (normally useful) string complexity by considering every input to be a
// just a slice of u8's.
fn bytewise_compare(first: &[u8], last: &[u8]) -> bool {
    if first.len() != last.len() {
        return false;
    }

    for (idx, c) in first.iter().enumerate() {
        if last[idx] != *c {
            return false;
        }
    }

    true
}

const LOOKUPS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn find_digit_str_forwards(input: &[u8]) -> Option<u32> {
    // fast path:
    if input.len() < 3 {
        return None;
    }

    for (comp, val) in LOOKUPS {
        if input.len() < comp.len() {
            continue;
        }

        if bytewise_compare(comp.as_bytes(), &input[..comp.len()]) {
            return Some(val);
        }
    }

    None
}

fn find_digit_str_backwards(input: &[u8]) -> Option<u32> {
    // fast path:
    if input.len() < 3 {
        return None;
    }

    for (comp, val) in LOOKUPS {
        if input.len() < comp.len() {
            continue;
        }

        if bytewise_compare(comp.as_bytes(), &input[(input.len() - comp.len())..]) {
            return Some(val);
        }
    }

    None
}

/// Two-pointer approach for finding the first and last digits in a string.
pub fn find_first_last(input: &'static str, recognize_strs: bool) -> Option<(u32, u32)> {
    if input.is_empty() {
        return None;
    }
    // because we only have to process ASCII characters, we can skip the whole UTF-8 codepoints
    // thing and just treat the input as an array of bytes, which allows our two-pointer approach to
    // be fairly fast.
    let bytes = input.as_bytes();

    let mut start_ptr = 0;
    let mut end_ptr = bytes.len() - 1;
    let mut first_digit: Option<u32> = None;
    let mut last_digit: Option<u32> = None;

    // Iterate forwards until we find the first digit (by some definition)
    while first_digit.is_none() && start_ptr < bytes.len() {
        let c = bytes[start_ptr];

        // We can't use convience methods like "is_digit()" because we converted the chars to bytes
        // for the sake of having indexable access to them.
        if c > 48 && c < 58 {
            first_digit = Some((c - 48) as u32);
            break;
        }

        if recognize_strs {
            if let Some(digit) = find_digit_str_forwards(&bytes[start_ptr..]) {
                first_digit = Some(digit);
                break;
            }
        }

        start_ptr += 1;
    }

    // Iterate backwards until we find the last digit (by some definition)
    while last_digit.is_none() {
        let c = bytes[end_ptr];

        if c > 48 && c < 58 {
            last_digit = Some((c - 48) as u32);
            break;
        }

        if recognize_strs {
            if let Some(digit) = find_digit_str_backwards(&bytes[..end_ptr + 1]) {
                last_digit = Some(digit);
                break;
            }
        }

        // We can't check end_ptr >= 0 in the loop condition, because it's a usize, so by definition
        // it could never be less than 0, and we want to avoid wrapping.
        if end_ptr == 0 {
            break;
        }

        end_ptr -= 1;
    }

    match (first_digit, last_digit) {
        (Some(first), Some(last)) => Some((first, last)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_last_when_both_defined() {
        let input_str = "abc1de3f2zxcv";
        let result = find_first_last(input_str, true);

        assert_eq!(result.unwrap(), (1, 2));
    }
    #[test]
    fn test_find_first_last_when_only_first() {
        let input_str = "1asbddfwerqe";
        let result = find_first_last(input_str, true);

        assert_eq!(result.unwrap(), (1, 1));
    }

    #[test]
    fn test_find_first_last_when_none() {
        let input_str = "asbddfwerqe";
        let result = find_first_last(input_str, true);

        assert!(result.is_none());
    }

    #[test]
    fn test_find_first_last_when_first_is_string() {
        let input_str = "one2";
        let result = find_first_last(input_str, true);

        assert_eq!(result.unwrap(), (1, 2));
    }

    #[test]
    fn test_find_first_last_when_last_is_string() {
        let input_str = "abone1eight";
        let result = find_first_last(input_str, true);

        assert_eq!(result.unwrap(), (1, 8));
    }

    #[test]
    fn test_not_recognize_strs() {
        let input_str = "abone2eight";
        let result = find_first_last(input_str, false);

        assert_eq!(result.unwrap(), (2, 2));
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(find_first_last("", true), None);
    }

    #[test]
    fn test_bytewise_compare_forwards() {
        let input_str = "abcone923";
        let first_slice = &input_str.as_bytes()[3..6];
        assert!(bytewise_compare(first_slice, "one".as_bytes()));
    }
}
