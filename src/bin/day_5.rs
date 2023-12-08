use std::str::Lines;
use itertools::Itertools;

fn main() {

}

struct ConversionRange {
    source: u32,
    destination: u32,
    length: u32
}

struct ConversionMap {
    ranges: Vec<ConversionRange>
}

impl ConversionMap {
    fn convert(&self, index: u32) -> u32 {
        for range in &self.ranges {
            let end = range.source + range.length;
            if index >= range.source && index < end {
                return range.destination + (index - range.source);
            }
        }

        index
    }
}

fn parse_data(data: &str) -> (Vec<u32>, Vec<ConversionMap>) {
    let mut data_iter = data.lines().into_iter();
    let seeds_str: String = data_iter.next().unwrap().chars().skip(7).collect();
    let seeds = seeds_str.split(" ").map(|seed_str| seed_str.parse::<u32>().unwrap()).collect();
    data_iter.next();

    let mut conversions = vec![];
    for _ in 0..7 {
        conversions.push(parse_map(&mut data_iter));
    }

    (seeds, conversions)
}

fn parse_map(data_iter: &mut Lines) -> ConversionMap {
    let mut ranges = vec![];
    data_iter.next();
    let mut line = data_iter.next().unwrap();

    while !line.is_empty() {
        let (source, destination, length) = line.split(" ").map(|num_str| num_str.parse::<u32>().unwrap()).collect_tuple().unwrap();
        ranges.push(ConversionRange{
            source,
            destination,
            length
        });

        line = data_iter.next().unwrap();
    }

    ConversionMap{
        ranges
    }
}

fn evaluate_location(seed: u32, conversions: &Vec<ConversionMap>) -> u32 {
    let mut value = seed;
    for conversion in conversions {
        value = conversion.convert(value);
    }
    value
}

#[cfg(test)]
mod tests {
    use crate::{evaluate_location, parse_data};

    #[test]
    fn task1() {
        let data = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4

";

        let (seeds, conversions) = parse_data(data);
        let locations: Vec<u32> = seeds.iter().map(|seed| evaluate_location(*seed, &conversions)).collect();

        assert_eq!(locations, vec![82, 43, 86, 35]);

        let min_location = locations.iter().min().unwrap();
        assert_eq!(min_location, &35);
    }
}