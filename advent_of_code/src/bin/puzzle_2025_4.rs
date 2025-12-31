const PAPER_ROLL_CHAR: char = '@';

fn main() {
    let input_str = include_str!("../../data/2025_4/input.txt");
    let result = part_1(input_str);
    println!("Part 1: {}", result);

    let result = part_2(input_str);
    println!("Part 2: {}", result);
}

fn part_1(input: &str) -> i32 {
    let map: Vec<Vec<bool>> = input.lines().map(
        |x| x.chars().map(|y| y == PAPER_ROLL_CHAR).collect()
    ).collect();

    let mut accessible_rolls = 0;
    for row in 0..map.len() {
        for column in 0..map[row].len() {
            if !map[row][column] { continue }
            let mut neighbour_count = 0;
            let mut left_to_check = 8;
            let mut stop = false;
            for row_neighbour in row.checked_sub(1).unwrap_or(0)..=row + 1 {
                for column_neighbour in column.checked_sub(1).unwrap_or(0)..=column + 1 {
                    if row_neighbour == row && column_neighbour == column {continue}
                    if row_neighbour < map.len()
                        && column_neighbour < map[row].len()
                        && map[row_neighbour][column_neighbour]
                    {neighbour_count += 1}

                    left_to_check -= 1;

                    if neighbour_count >= 4 || neighbour_count + left_to_check < 4 {
                        stop = true;
                        break
                    }
                }
                if stop {break}
            }
            if neighbour_count < 4 {accessible_rolls += 1}
        }
    }
    accessible_rolls
}

fn remove(mut map: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    for row in 0..map.len() {
        for column in 0..map[row].len() {
            if !map[row][column] { continue }
            let mut neighbour_count = 0;
            let mut left_to_check = 8;
            let mut stop = false;
            for row_neighbour in row.checked_sub(1).unwrap_or(0)..=row + 1 {
                for column_neighbour in column.checked_sub(1).unwrap_or(0)..=column + 1 {
                    if row_neighbour == row && column_neighbour == column {continue}
                    if row_neighbour < map.len()
                        && column_neighbour < map[row].len()
                        && map[row_neighbour][column_neighbour]
                    {neighbour_count += 1}

                    left_to_check -= 1;

                    if neighbour_count >= 4 || neighbour_count + left_to_check < 4 {
                        stop = true;
                        break
                    }
                }
                if stop {break}
            }
            if neighbour_count < 4 {map[row][column] = false}
        }
    }
    map
}

fn part_2(input: &str) -> i64 {
    let mut map: Vec<Vec<bool>> = input.lines().map(
        |x| x.chars().map(|y| y == PAPER_ROLL_CHAR).collect()
    ).collect();

    let original_accessible_rolls: i64 = map.iter().fold(
        0, |val, row| val + row.iter().fold(
            0, |val, cell| val + (if *cell {1} else {0})
        )
    );
    let mut previous_accessible_rolls = original_accessible_rolls;

    loop {
        map = remove(map);
        let current_accessible_rolls: i64 = map.iter().fold(
            0, |val, row| val + row.iter().fold(
                0, |val, cell| val + (if *cell {1} else {0})
            )
        );
        if current_accessible_rolls == previous_accessible_rolls {
            previous_accessible_rolls = current_accessible_rolls;
            break
        }
        previous_accessible_rolls = current_accessible_rolls;
    }
    original_accessible_rolls - previous_accessible_rolls
}

// struct Space {
//     map: Vec<Vec<Space>>,
//     row: usize,
//     col: usize,
//     is_roll: bool,
//     removable: bool,
// }
//
// impl Space {
//     fn get_neighbours(&self) -> Vec<Space> {
//         let mut neighbours: Vec<Space> = Vec::new();
//         for row_neighbour in self.row.checked_sub(1).unwrap_or(0)..=self.row + 1 {
//             for column_neighbour in self.col.checked_sub(1).unwrap_or(0)..=self.col + 1 {
//                 if row_neighbour == self.row && column_neighbour == self.col { continue }
//                 if row_neighbour < self.map.len()
//                     && column_neighbour < self.map[self.row].len()
//                     && self.map[row_neighbour][column_neighbour].is_roll
//                 { neighbours.push(self.map[row_neighbour][column_neighbour]) }
//             }
//         }
//         neighbours
//     }
//
//     fn remove(&mut self) -> bool {
//         let neighbours = self.get_neighbours();
//         if neighbours.len() < 4 {
//             self.removable = true;
//             true
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../../data/2025_4/test.txt");
        let result = part_1(input);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../../data/2025_4/test.txt");
        let result = part_2(input);
        assert_eq!(result, 43);
    }
}
