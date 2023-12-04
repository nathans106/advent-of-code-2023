use regex::Regex;

fn main() {
    let data = include_str!("../../data/day_4.txt");
    let cards = parse_data(data);
    let values = calculate_values(&cards);
    let values_sum : u32 = values.iter().sum();
    println!("Task 1: {values_sum}");
}

struct Card {
    winners: Vec<u32>,
    yours: Vec<u32>
}

fn parse_data(data: &str) -> Vec<Card> {
    let split_re = Regex::new(r"Card \d+: ([\s\d]+) \| ([\s\d]+)").unwrap();
    let nums_re = Regex::new(r" (\d+)").unwrap();

    split_re.captures_iter(data).map(|split_capture| {
        let (_, [winners_str, yours_str]) = split_capture.extract();

        let winners : Vec<u32> = nums_re.captures_iter(winners_str).map(|nums_capture| {
            nums_capture[1].parse::<u32>().unwrap()
        }).collect();

        let yours = nums_re.captures_iter(yours_str).map(|nums_capture| {
            nums_capture[1].parse::<u32>().unwrap()
        }).collect();

        Card{winners, yours}
    }).collect()
}

fn calculate_values(cards: &Vec<Card>) -> Vec<u32> {
    cards.iter().map(|card| {
        let mut value = 0;

        for num in &card.yours {
            if card.winners.contains(num) {
                if value == 0 {
                    value = 1;
                } else {
                    value = value * 2;
                }
            }
        }

        value
    }).collect()
}

#[cfg(test)]
mod tests {
    use crate::{calculate_values, parse_data};

    #[test]
    fn task1() {
        let data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let cards = parse_data(data);
        let values = calculate_values(&cards);
        //let values_sum : u32 = values.iter().sum();
        //assert_eq!(values_sum, 13);
    }
}