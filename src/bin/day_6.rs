use itertools::Itertools;

fn main() {
    let data = include_str!("../../data/day_6.txt");
    let races = parse_multiple_races(data);
    let answer: u64 = races.iter().map(|race| race.num_win_scenarios()).product();
    println!("Task 1: {answer}");

    let race = parse_single_race(data);
    let answer: u64 = race.num_win_scenarios();
    println!("Task 2: {answer}");
}

struct Race {
    time: u64,
    record: u64
}

impl Race {
    fn distance(&self, hold_time: u64) -> u64 {
        let move_time = self.time - hold_time;
        move_time * hold_time
    }

    fn beat_record(&self, hold_time: u64) -> bool {
        self.distance(hold_time) > self.record
    }

    fn num_win_scenarios(&self) -> u64 {
        let mut lower_win = None;
        let mut upper_loss = None;

        for hold_time in 1..self.time {
            if self.beat_record(hold_time) {
                match lower_win {
                    None => lower_win = Some(hold_time),
                    Some(_) => ()
                }
            } else {
                match lower_win {
                    None => (),
                    Some(_) => {
                        upper_loss = Some(hold_time);
                        break;
                    }
                }
            }
        }

        upper_loss.unwrap() - lower_win.unwrap()
    }
}

fn parse_line_multiple(line: &str) -> Vec<u64> {
    let nums_str = line.chars().skip(11).collect::<String>();
    nums_str.split_whitespace().map(|num_str| num_str.parse::<u64>().unwrap()).collect_vec()
}

fn parse_multiple_races(data: &str) -> Vec<Race> {
    let mut data_iter = data.lines();
    let times = parse_line_multiple(data_iter.next().unwrap());
    let distances = parse_line_multiple(data_iter.next().unwrap());
    times.into_iter().zip(distances).map(|(time, record)| Race{time, record}).collect()
}

fn parse_line_single(line: &str) -> u64 {
    let nums_str = line.chars().skip(11).collect::<String>();
    let num_str = nums_str.split_whitespace().collect::<String>();
    num_str.parse().unwrap()
}

fn parse_single_race(data: &str) -> Race {
    let mut data_iter = data.lines();
    let time = parse_line_single(data_iter.next().unwrap());
    let record = parse_line_single(data_iter.next().unwrap());
    Race {
        time,
        record
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_multiple_races, parse_single_race};

    #[test]
    fn task1() {
        let data = "Time:      7  15   30
Distance:  9  40  200";

        let races = parse_multiple_races(data);
        let answer: u64 = races.iter().map(|race| race.num_win_scenarios()).product();
        assert_eq!(answer, 288);
    }

    #[test]
    fn task2() {
        let data = "Time:      7  15   30
Distance:  9  40  200";

        let race = parse_single_race(data);
        let answer: u64 = race.num_win_scenarios();
        assert_eq!(answer, 71503);
    }
}
