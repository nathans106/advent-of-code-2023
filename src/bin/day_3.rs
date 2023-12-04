extern crate core;

use core::num::fmt::Part::Num;

fn main() {
    let data = include_str!("../../data/day3.txt");
    let (nums, part_map) = parse_data(&data);
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


    for x in (0..num_rows) {
        let row = data.lines().nth(x).unwrap();
        let mut last_num = Option::None;


        for y in (0..num_columns) {
            let c = row.chars().nth(y).unwrap();
            let maybe_digit = c.to_digit(10);
            match maybe_digit {
                Some(digit) => {
                    match last_num {
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
                            numbers.append(num);
                            last_num = None;
                        },
                        None => ()
                    }

                    if c != "." {
                        part_matrix[x][y] = true;
                    }
                }
            }
        }
    }

    (numbers, part_matrix)
}

#[cfg(test)]
mod tests {
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


    }

    fn task2() {

    }
}