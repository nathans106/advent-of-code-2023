fn main() {
    let data = include_str!("../../data/day1.txt");
    let digits = first_last_digits(data);
    let sum = sum_digits(&digits);
    println!("Task 1: {sum}")
}

fn first_last_digits(data: &str) -> Vec<u32> {
    data.lines().map(|line| {
        let first = line.chars().find(|c| return c.to_digit(10).is_some()).unwrap().to_digit(10).unwrap();
        let last = line.chars().rfind(|c| return c.to_digit(10).is_some()).unwrap().to_digit(10).unwrap();
        (first * 10) + last
    }).collect()
}

fn sum_digits(digits: &Vec<u32>) -> u32 {
    return digits.iter().sum();
}

#[cfg(test)]
mod tests {
    use crate::{first_last_digits, sum_digits};

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
}