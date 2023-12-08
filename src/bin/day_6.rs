use itertools::Itertools;

fn main() {
    let data = include_str!("../../data/day_6.txt");
    let races = parse_data(data);
    let answer: u32 = races.iter().map(|race| race.num_win_scenarios()).product();
    println!("Task 1: {answer}");
}

struct Race {
    time: u32,
    record: u32
}

impl Race {
    fn distance(&self, hold_time: u32) -> u32 {
        let move_time = self.time - hold_time;
        move_time * hold_time
    }

    fn beat_record(&self, hold_time: u32) -> bool {
        self.distance(hold_time) > self.record
    }

    fn num_win_scenarios(&self) -> u32 {
        let mut lower_win = None;
        let mut upper_loss = None;

        for hold_time in 1..self.time {
            if self.beat_record(hold_time) {
                match(lower_win) {
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

fn parse_line(line: &str) -> Vec<u32> {
    let nums_str = line.chars().skip(11).collect::<String>();
    nums_str.split_whitespace().map(|num_str| num_str.parse::<u32>().unwrap()).collect_vec()
}

fn parse_data(data: &str) -> Vec<Race> {
    let mut data_iter = data.lines();
    let times = parse_line(data_iter.next().unwrap());
    let distances = parse_line(data_iter.next().unwrap());
    times.into_iter().zip(distances).map(|(time, record)| Race{time, record}).collect()
}

#[cfg(test)]
mod tests {
    use crate::parse_data;

    #[test]
    fn task1() {
        let data = "Time:      7  15   30
Distance:  9  40  200";

        let races = parse_data(data);
        let answer: u32 = races.iter().map(|race| race.num_win_scenarios()).product();
        assert_eq!(answer, 288);
    }
}
