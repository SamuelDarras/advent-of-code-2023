use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input2.txt");
    let result = part2(input);
    println!("{result}");
}

fn part2(input: &str) -> String {
    let mut hands = input.lines().map(Hand::parse).collect::<Vec<_>>();
    hands.sort();
    let res = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| {
            println!(
                "{} {:?} => {}\t{hand:?}",
                rank + 1,
                hand.bid,
                (rank + 1) * hand.bid
            );
            (rank + 1) * hand.bid
        })
        .sum::<usize>();
    res.to_string()
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand {
    cards: Vec<usize>,
    before_cards: Vec<usize>,
    bid: usize,
}
impl Hand {
    fn parse(input: impl Into<String>) -> Self {
        let input: String = input.into();
        let splitted = input.split_whitespace().collect::<Vec<&str>>();
        let before_cards = splitted[0]
            .chars()
            .map(|v| match v {
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'T' => 10,
                'J' => 1,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        let bid = splitted[1].parse::<usize>().unwrap();

        let cards = Self::calc_best_cards(before_cards.clone());

        Self {
            before_cards,
            cards,
            bid,
        }
    }

    fn calc_best_cards(cards: Vec<usize>) -> Vec<usize> {
        let mut positions = Vec::new();
        for i in 0..cards.len() {
            if cards[i] == 1 {
                positions.push(i);
            }
        }
        println!("{cards:?}");
        println!("{positions:?}");
        println!(
            "{:?}",
            (0..positions.len()).map(|_| 1..=13).collect::<Vec<_>>()
        );
        let res = (0..positions.len())
            .map(|_| 1..=14)
            .multi_cartesian_product()
            .map(|t| {
                // println!("{t:?}");
                t.into_iter().zip(positions.clone()).collect::<Vec<_>>()
            })
            .map(|v| {
                let mut cards = cards.clone();
                for (replace, idx) in v {
                    cards[idx] = replace;
                }
                Hand {
                    cards,
                    before_cards: Vec::new(),
                    bid: 0,
                }
            })
            .map(|hand| {
                let r = (hand.cards.clone(), hand.power());
                r
            })
            .max_by(|(_, power_a), (_, power_b)| power_a.cmp(power_b));

        println!("{res:?}");

        if let Some((res, _)) = res {
            res
        } else {
            cards
        }
    }

    fn power(&self) -> usize {
        let mut counts = HashMap::new();
        self.cards.iter().for_each(|card| {
            counts.insert(card, self.cards.iter().filter(|c| *c == card).count());
        });
        let uniq_counts = counts.values().map(|v| *v).collect::<Vec<usize>>();

        if uniq_counts.contains(&5) {
            return 6;
        }
        if uniq_counts.contains(&4) {
            return 5;
        }
        if uniq_counts.contains(&3) && uniq_counts.contains(&2) {
            return 4;
        }
        if uniq_counts.contains(&3) {
            return 3;
        }
        if uniq_counts.contains(&2) {
            let mut cp = uniq_counts.clone();
            let mut removed = false;
            cp.retain(|&v| {
                if removed || v != 2 {
                    true
                } else {
                    removed = true;
                    false
                }
            });
            return if cp.contains(&2) { 2 } else { 1 };
        }

        0
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        print!("{:?} {:?} ", self, other);
        if self.power() == other.power() {
            for (this_card, other_card) in self.before_cards.iter().zip(other.before_cards.iter()) {
                print!("{this_card:?} ~ {other_card:?}");
                if this_card != other_card {
                    let r = this_card.cmp(&other_card);
                    println!(" => {r:?}");
                    return r;
                }
                println!("");
            }
            std::cmp::Ordering::Equal
        } else {
            let r = self.power().cmp(&other.power());
            println!(" => {r:?}");
            r
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;

    #[test]
    fn test_example() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

        assert_eq!(part2(input), "5905");
    }

    #[test]
    fn test_example_2() {
        let input = "2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41";

        assert_eq!(part2(input), "6839");
    }

    #[test]
    fn test_foak_with_js_is_weaker_than_a_normal_one() {
        let h1 = Hand {
            cards: vec![13, 13, 13, 13, 13],
            before_cards: vec![13, 13, 1, 13, 13],
            bid: 0,
        };
        let h2 = Hand {
            cards: vec![13, 13, 13, 13, 13],
            before_cards: vec![13, 13, 13, 13, 13],
            bid: 0,
        };
        assert_eq!(h1.cmp(&h2), Ordering::Less);
    }
}
