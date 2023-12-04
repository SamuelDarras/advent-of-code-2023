use std::iter::Sum;

fn main() {
    let input = include_str!("./input1.txt");
    let result = part1(input);
    println!("{result}");
}

fn part1(input: &str) -> String {
    let res = input.lines().map(|card| one_card(card)).sum::<u32>();

    res.to_string()
}

fn one_card(input: &str) -> u32 {
    let numbers = input.split(":").last().unwrap();
    let mut numbers = numbers.split("|");
    let winning_numbers = numbers
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let my_numbers = numbers
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let winning_count = my_numbers
        .iter()
        .filter(|n| winning_numbers.contains(n))
        .count();

    if winning_count >= 1 {
        2u32.pow(winning_count as u32 - 1)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

        assert_eq!(part1(input), "13");
    }
}
