fn main() {
    let input = include_str!("./input2.txt");
    let result = part1(input);
    println!("{result}");
}

fn part1(input: &str) -> String {
    let mut lines = input.lines();
    let times = lines.next().unwrap();
    let records = lines.next().unwrap();
    let times = times
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();
    let records = records
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    times
        .iter()
        .map(|time| (time, time.div_euclid(2)))
        .zip(records.iter())
        .map(|((time, h_time), record)| {
            (0usize..=h_time)
                .filter(|&v| {
                    let r = v * (time - v);
                    r > *record
                })
                .count()
                * 2
                - if time & 1 == 0 { 1 } else { 0 }
        })
        .product::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "Time:      7  15   30
Distance:  9  40  200
";

        assert_eq!(part1(input), "288");
    }
}
