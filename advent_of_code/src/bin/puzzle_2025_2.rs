use std::collections::HashSet;

fn main() {
    let input_str = include_str!("../../data/2025_2/input.txt");
    let result = part_1(input_str);
    println!("Part 1: {}", result);

    let result = part_2(input_str);
    println!("Part 2: {}", result);
}

// fn is_invalid_regex(value: i64) -> bool {
//     // Far too slow
//     let value_str = value.to_string();
//     Regex::new(r"^(\d+)\1$").unwrap().is_match(&value_str).unwrap()
// }

fn is_invalid_part_1(value: i64) -> bool {
    let value_str = value.to_string();
    if value_str.len() % 2 != 0 {
        return false
    }
    if value_str[..(value_str.len()/2)] == value_str[(value_str.len()/2)..] {
        return true
    }
    false
}

fn is_invalid_part_2(value: i64) -> bool {
    let value_str = value.to_string();
    let length = value_str.len();
    for segment_len in 1..((length / 2) + 1) {
        if length % segment_len != 0 {
            continue
        }
        if length == segment_len {
            continue
        }
        let repeats = length / segment_len;
        if value_str[..segment_len].repeat(repeats) == value_str {
            return true
        }
    }
    false
}

fn part_1(input_str: &str) -> i64 {
    let mut invalids: HashSet<i64>  = HashSet::new();
    let mut checked: HashSet<i64> = HashSet::new();

    let ranges = input_str.split(",");
    for range in ranges {
        let split_range: Vec<&str> = range.trim().split("-").collect();
        let start: i64 = split_range[0].parse().unwrap();
        let end : i64 = split_range[1].parse().unwrap();
        for i in start..(end + 1) {
            if !checked.contains(&i) {
                if is_invalid_part_1(i) {
                    invalids.insert(i);
                }
                checked.insert(i);
            }
        }
    }

    invalids.iter().sum()
}

fn part_2(input_str: &str) -> i64 {
    let mut invalids: HashSet<i64>  = HashSet::new();
    let mut checked: HashSet<i64> = HashSet::new();

    let ranges = input_str.split(",");
    for range in ranges {
        let split_range: Vec<&str> = range.trim().split("-").collect();
        let start: i64 = split_range[0].parse().unwrap();
        let end : i64 = split_range[1].parse().unwrap();
        for i in start..(end + 1) {
            if !checked.contains(&i) {
                if is_invalid_part_2(i) {
                    invalids.insert(i);
                }
                checked.insert(i);
            }
        }
    }

    invalids.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_1234() {
        let input = 1234;
        let result = is_invalid_part_1(input);
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_invalid_123412341234() {
        let input = 12341234;
        let result = is_invalid_part_1(input);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_valid_1234_part_2() {
        let input = 1234;
        let result = is_invalid_part_2(input);
        assert_eq!(result, false);
    }

    #[test]
    fn test_is_invalid_12341234_part_2() {
        let input = 12341234;
        let result = is_invalid_part_2(input);
        assert_eq!(result, true);
    }

    #[test]
    fn test_is_valid_1234123_part_2() {
        let input = 1234123;
        let result = is_invalid_part_2(input);
        assert_eq!(result, false);
    }

    #[test]
    fn test_part_1_1227775554() {
        let input = include_str!("../../data/2025_2/test.txt");
        let result = part_1(input);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_part_2_4174379265() {
        let input = include_str!("../../data/2025_2/test.txt");
        let result = part_2(input);
        assert_eq!(result, 4174379265);
    }
}