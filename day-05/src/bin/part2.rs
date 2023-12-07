use std::ops::Range;

use rayon::prelude::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};

fn main() {
    let input = include_str!("./input1.txt");
    let result = part1(input);
    println!("{result}");
}

fn part1(input: &str) -> String {
    let almanac = parse_maps(input);
    let res = almanac
        .seeds
        .chunks(2)
        .map(|chunk| {
            println!("{chunk:?}");
            let min = (*chunk.first().unwrap()..*chunk.first().unwrap() + *chunk.last().unwrap())
                .into_iter()
                .par_bridge()
                .map(|seed| almanac.follow_map(seed))
                .min()
                .unwrap();
            println!("=> {min}");
            min
        })
        .min()
        .unwrap();

    println!("{res:?}");

    res.to_string()
}

fn parse_maps(input: &str) -> Almanac {
    let mut lines = input.lines().peekable();
    let seeds = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    lines.next();
    let mut maps = Vec::new();
    while lines.peek().is_some() {
        maps.push(parse_one_map(&mut lines));
    }
    Almanac { seeds, maps }
}

fn parse_one_map<'s>(
    lines: &mut impl Iterator<Item = &'s str>,
) -> Vec<(Range<usize>, Range<usize>)> {
    let name = lines.next().unwrap();
    // println!("{name}");
    let mut line = lines.next().unwrap();

    let mut r = Vec::new();
    while line != "" {
        let values = line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let dst_begin = values[0];
        let src_begin = values[1];
        let length = values[2] - 1;
        let src_range = src_begin..src_begin + length + 1;
        let dst_range = dst_begin..dst_begin + length + 1;
        // println!("{src_range:?} -> {dst_range:?}");
        r.push((src_range, dst_range));
        if let Some(l) = lines.next() {
            line = l;
        } else {
            return r;
        }
    }
    r
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Vec<(Range<usize>, Range<usize>)>>,
}
impl Almanac {
    fn follow_map(&self, value: usize) -> usize {
        let mut value = value;
        for i in 0..self.maps.len() {
            value = self.map(i, value);
        }
        value
    }

    fn map(&self, map_idx: usize, value: usize) -> usize {
        let ranges = &self.maps[map_idx];
        let mut res = value;
        for range in ranges {
            if range.0.contains(&value) {
                res = (value as isize + (range.1.start as isize - range.0.start as isize)) as usize;
                break;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_almanac_map() {
        let almanac = Almanac {
            seeds: Vec::new(),
            maps: vec![vec![(0..10, 20..30)]],
        };

        let res = almanac.map(0, 5);
        assert_eq!(res, 25);
        let res = almanac.map(0, 11);
        assert_eq!(res, 11);
    }

    #[test]
    fn test_example() {
        let input = "seeds: 79 14 55 13

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

        assert_eq!(part1(input), "46");
    }
}
