fn main() {
    let input_str = include_str!("../../data/2025_3/input.txt");
    let result = part_1(input_str);
    println!("Part 1: {}", result);

    let result = part_2(input_str);
    println!("Part 2: {}", result);
}

fn part_1(input: &str) -> i32 {
    let mut total = 0;
    for line in input.lines() {
        total += get_highest_joltage(line);
    }
    total
}

fn part_2(input: &str) -> i64 {
    let mut total: i64 = 0;
    for line in input.lines() {
        total += get_highest_joltage_from_n(line, 12);
    }
    total
}

fn get_highest_joltage(line: &str) -> i32 {
    let mut highest_first_digit = 0;
    let mut highest_first_digit_index = 0;
    for (index, digit) in line[..(line.len() - 1)].chars().enumerate() {
        let digit_int: u32 = digit.to_digit(10).unwrap();
        if digit_int > highest_first_digit {
            highest_first_digit = digit_int;
            highest_first_digit_index = index;
        }
        if highest_first_digit == 9 {
            break
        }
    }
    let mut highest_second_digit: u32 = 0;
    for digit in line[(highest_first_digit_index + 1)..line.len()].chars() {
        let digit_int: u32 = digit.to_digit(10).unwrap();
        if digit_int > highest_second_digit {
            highest_second_digit = digit_int;
        }
        if highest_second_digit == 9 {
            break
        }
    }

    let result: i32 = format!("{}{}", highest_first_digit, highest_second_digit).parse().unwrap();
    result
}

fn get_highest_joltage_from_n(line: &str, size: u32) -> i64 {
    let mut result: Vec<String> = Vec::with_capacity(size as usize);
    let mut current_leftmost_index: i32 = -1;
    for _ in 0..size {
        let mut current_highest_val = 0;
        let digits_left = size - result.len() as u32;
        let highest_index_to_check: usize = line.len() - digits_left as usize + 1;
        for index in (current_leftmost_index + 1) as usize..highest_index_to_check {
            let input_digit: u32 = line.chars().nth(index).unwrap().to_digit(10).unwrap();
            if input_digit > current_highest_val {
                current_highest_val = input_digit;
                current_leftmost_index = index as i32;
            }
            if current_highest_val == 9 {
                break
            }
        }
        result.push(current_highest_val.to_string());
    }

    let result_int: i64 = result.join("").parse().unwrap();
    result_int
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_val() {
        let input = "116572589357";
        let result = get_highest_joltage(input);
        assert_eq!(result, 97);
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("../../data/2025_3/test.txt");
        let result = part_1(input);
        assert_eq!(result, 357);
    }

    #[test]
    fn test_correct_val_n_12() {
        let input = "987654321111111";
        let result = get_highest_joltage_from_n(input, 12);
        assert_eq!(result, 987654321111);
    }

    #[test]
    fn test_correct_val_n_4() {
        let input = "115941";
        let result = get_highest_joltage_from_n(input, 4);
        assert_eq!(result, 5941);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../../data/2025_3/test.txt");
        let result = part_2(input);
        assert_eq!(result, 3121910778619);
    }
}
