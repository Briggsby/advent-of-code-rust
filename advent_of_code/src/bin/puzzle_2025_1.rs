const BACKWARDS_CHAR: &str = "L";

fn main() {
    let input_str = include_str!("../../data/2025_1/input.txt");
    let result = part_1(input_str);
    println!("Part 1: {}", result);

    let result = part_2(input_str);
    println!("Part 2: {}", result);
}

fn part_1(input_str: &str) -> i32 {
    let lines = input_str.lines();
    let mut value = 50;
    let mut zero_count = 0;

    for line in lines {
        let direction_str = &line[..1];
        let direction = if direction_str == BACKWARDS_CHAR {1} else {-1};
        let change: i32 = (&line[1..]).parse().unwrap();
        value = (value + (direction * change) + 100) % 100;
        if value == 0 { zero_count += 1};
    }
    zero_count
}

fn part_2(input_str: &str) -> i32 {
    let lines = input_str.lines();
    let mut value = 50;
    let mut zero_count = 0;

    for line in lines {
        let direction_str = &line[..1];
        let direction = if direction_str == BACKWARDS_CHAR {-1} else {1};
        let mut change: i32 = (&line[1..]).parse().unwrap();

        if change >= 100 {
            zero_count += change / 100;
            change = change % 100;
        }

        let prev_value = value;
        let raw_value = value + (direction * change);
        if prev_value != 0 && (raw_value >= 100 || raw_value <= 0) {
            zero_count += 1;
        }

        value = (raw_value + 100) % 100;
    }
    zero_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_3() {
        let input = include_str!("../../data/2025_1/test.txt");
        let result = part_1(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_2_6() {
        let input = include_str!("../../data/2025_1/test.txt");
        let result = part_2(input);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part_2_3() {
        let input = "L50\nR200\nR5";
        let result = part_2(input);
        assert_eq!(result, 3);
    }
}