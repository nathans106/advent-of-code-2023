extern crate core;

fn main() {
    let data = include_str!("../../data/day3.txt");
    let (nums, part_matrix) = parse_data(&data);
    let part_nums = find_part_nums(nums, &part_matrix);
    let parts_sum : u32 = part_nums.iter().map(|num| num.value).sum();
    println!("Task 1: {parts_sum}");
}

struct Number {
    value: u32,
    row: usize,
    first_column: usize,
    length: usize
}

fn parse_data(data: &str) -> (Vec<Number>, Vec<Vec<bool>>) {
    let num_rows = data.lines().count();
    let num_columns = data.lines().nth(0).unwrap().chars().count();

    let mut numbers = vec![];
    let mut part_matrix = vec![vec![false; num_columns]; num_rows];


    for x in 0..num_rows {
        let row = data.lines().nth(x).unwrap();
        let mut last_num : Option::<Number> = Option::None;

        for y in 0..num_columns {
            let c = row.chars().nth(y).unwrap();
            let maybe_digit = c.to_digit(10);
            match maybe_digit {
                Some(digit) => {
                    match &mut last_num {
                        Some(num) => {
                            num.value = (num.value * 10) + digit;
                            num.length += 1;
                        },
                        None => {
                            last_num = Some(Number {value: digit,
                            row : x,
                            first_column: y,
                                length : 1
                            });
                        }
                    }
                },
                None => {
                    match last_num {
                        Some(num) => {
                            numbers.push(num);
                            last_num = None;
                        },
                        None => ()
                    }

                    if c != '.' {
                        part_matrix[x][y] = true;
                    }
                }
            }
        }
    }

    (numbers, part_matrix)
}

fn find_part_nums(nums: Vec<Number>, part_matrix: &Vec<Vec<bool>>) -> Vec<Number> {
    let num_rows = part_matrix.len();
    let num_cols = part_matrix.first().unwrap().len();

    nums.into_iter().filter(|num| {
        let left_most_col = if num.first_column >= 1 {num.first_column - 1} else {0};
        let right_most_col = if num.first_column + num.length > num_cols - 1 {num.first_column + num.length} else {num_cols - 1};

        // Top row
        if num.row >= 1 {
            let top_row = num.row - 1;

            for c in left_most_col..right_most_col + 1 {
                if part_matrix[top_row][c] {
                    return true;
                }
            }
        }

        // Left Side
        if num.first_column >= 1 && part_matrix[num.row][num.first_column - 1] {
            return true;
        }

        // Right side
        let column_past_end = num.first_column + num.length;
        if column_past_end < num_cols && part_matrix[num.row][column_past_end] {
            return true;
        }

        // Bottom row
        if num.row < num_rows - 1 {
            let bottom_row = num.row + 1;

            for c in left_most_col..right_most_col + 1 {
                if part_matrix[bottom_row][c] {
                    return true;
                }
            }
        }

        return false
    }).collect()
}

#[cfg(test)]
mod tests {
    use crate::{find_part_nums, parse_data};

    #[test]
    fn task1() {
        let data = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let (nums, part_matrix) = parse_data(&data);
        let part_nums = find_part_nums(nums, &part_matrix);
        let parts_sum : u32 = part_nums.iter().map(|num| num.value).sum();
        assert_eq!(parts_sum, 4361);
    }

    fn task2() {

    }
}