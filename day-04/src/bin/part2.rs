use std::{collections::VecDeque, iter::Sum};

fn main() {
    let input = include_str!("./input1.txt");
    let result = part2(input);
    println!("{result}");
}

fn part2(input: &str) -> String {
    let mut cards = VecDeque::from(input.lines().map(one_card).collect::<Vec<_>>());
    let mut cards_count = cards.len();
    while let Some(currnet_card) = cards.pop_front() {
        let count = currnet_card.get_winning();
        for card in cards.range_mut(0..count) {
            cards_count += currnet_card.instances;
            card.instances += currnet_card.instances;
        }
        // println!(
        //     "{:?}",
        //     cards
        //         .iter()
        //         .map(|c| (c.id, c.instances))
        //         .collect::<Vec<_>>()
        // );
    }
    cards_count.to_string()
}

fn one_card(input: &str) -> Card {
    let mut id_and_numbers = input.split(":");
    let id = id_and_numbers.next().unwrap()[5..].trim().parse().unwrap();
    let numbers = id_and_numbers.next().unwrap();
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

    Card {
        instances: 1,
        id,
        winning_numbers,
        my_numbers,
    }
}

#[derive(Clone, Debug)]
struct Card {
    instances: usize,
    id: usize,
    winning_numbers: Vec<usize>,
    my_numbers: Vec<usize>,
}
impl Card {
    fn get_winning(&self) -> usize {
        self.my_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count()
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

        assert_eq!(part2(input), "30");
    }
}
