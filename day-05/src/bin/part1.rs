use std::ops::Range;

fn main() {
    let input = include_str!("./input1.txt");
    let result = part1(input);
    println!("{result}");
}

fn part1(input: &str) -> String {
    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    println!("{seeds:?}");
    lines.next();

    parse_one_map(&mut lines);

    "".to_string()
}

fn parse_one_map(lines: &mut std::str::Lines<'_>) -> Vec<(Range<usize>, Range<usize>)> {
    let name = lines.next().unwrap();
    let mut line = lines.next().unwrap();

    let mut r = Vec::new();
    while line != "" {
        println!("{line}");
        let values = line
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let dst_begin = values[0];
        let src_begin = values[1];
        let length = values[2] - 1;
        let src_range = src_begin..src_begin + length;
        let dst_range = dst_begin..dst_begin + length;
        println!("{src_range:?} -> {dst_range:?}");
        line = lines.next().unwrap();
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(part1(input), "0");
    }
}
