use std::collections::HashMap;

fn main() {
    let data = include_str!("../../data/day1.txt");
    let digits = first_last_digits(data);
    let sum = sum_digits(&digits);
    println!("Task 1: {sum}");

    let digits2 = first_last_digits_or_nums(data);
    let sum2 = sum_digits(&digits2);
    println!("Task 2: {sum2}");
}

fn first_last_digits(data: &str) -> Vec<u32> {
    data.lines().map(|line| {
        let first = line.chars().find(|c| return c.to_digit(10).is_some()).unwrap().to_digit(10).unwrap();
        let last = line.chars().rfind(|c| return c.to_digit(10).is_some()).unwrap().to_digit(10).unwrap();
        (first * 10) + last
    }).collect()
}

fn first_last_digit_or_num(line: &str) -> u32 {
    let nums_to_digits = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9)
    ]);

    let mut l = 1;
    let mut first = None;
    while first.is_none() {
        let num: String = line.chars().take(l).collect();
        for key in nums_to_digits.keys() {
            if num.contains(key) {
                first = Some(nums_to_digits[key]);
                break;
            }
        }
        l += 1;
    }

    l=0;

    let mut last = None;
    while last.is_none() {
        let rnum: String = line.chars().rev().take(l).collect();
        let num: String = rnum.chars().rev().collect();
        for key in nums_to_digits.keys() {
            if num.contains(key) {
                last = Some(nums_to_digits[key]);
                break;
            }
        }
        l += 1;
    }

    let first_digit = first.unwrap();
    let last_digit = last.unwrap();
    (first_digit * 10) + last_digit
}

fn first_last_digits_or_nums(data: &str) -> Vec<u32> {
    data.lines().map(|line| first_last_digit_or_num(line)).collect()
}

fn sum_digits(digits: &Vec<u32>) -> u32 {
    return digits.iter().sum();
}

#[cfg(test)]
mod tests {
    use crate::{first_last_digits, first_last_digits_or_nums, sum_digits};

    #[test]
    fn task1() {
        let data = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let digits = first_last_digits(data);
        let sum = sum_digits(&digits);
        assert_eq!(sum, 142);
    }

    #[test]
    fn task2() {
        let data = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let digits = first_last_digits_or_nums(data);
        let sum = sum_digits(&digits);
        assert_eq!(sum, 281);
    }
}
