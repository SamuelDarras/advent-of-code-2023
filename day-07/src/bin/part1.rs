use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let result = part1(input);
    println!("{result}");
}

fn part1(input: &str) -> String {
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
    bid: usize,
}
impl Hand {
    fn parse(input: impl Into<String>) -> Self {
        let input: String = input.into();
        let splitted = input.split_whitespace().collect::<Vec<&str>>();
        let cards = splitted[0]
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
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        let bid = splitted[1].parse::<usize>().unwrap();

        Self { cards, bid }
    }

    fn power(&self) -> usize {
        let mut counts = HashMap::new();
        self.cards.iter().for_each(|card| {
            counts.insert(card, self.cards.iter().filter(|c| *c == card).count());
        });
        let uniq_counts = counts.values().map(|v| *v).collect::<Vec<usize>>();

        println!("{uniq_counts:?}");

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
            for (this_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
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

        assert_eq!(part1(input), "6440");
    }

    #[test]
    fn test_hand_power_five_of_a_kind() {
        let h = Hand {
            cards: vec![1, 1, 1, 1, 1],
            bid: 0,
        };
        assert_eq!(h.power(), 6);
    }

    #[test]
    fn test_hand_power_four_of_a_kind() {
        let h = Hand {
            cards: vec![1, 1, 1, 1, 2],
            bid: 0,
        };
        assert_eq!(h.power(), 5);
    }

    #[test]
    fn test_hand_power_three_of_a_kind() {
        let h = Hand {
            cards: vec![1, 1, 1, 2, 3],
            bid: 0,
        };
        assert_eq!(h.power(), 3);
    }

    #[test]
    fn test_hand_power_fullhouse() {
        let h = Hand {
            cards: vec![1, 1, 1, 2, 2],
            bid: 0,
        };
        assert_eq!(h.power(), 4);
    }

    #[test]
    fn test_hand_power_two_pair() {
        let h = Hand {
            cards: vec![13, 13, 12, 12, 1],
            bid: 0,
        };
        assert_eq!(h.power(), 2);
    }

    #[test]
    fn test_hand_power_one_pair() {
        let h = Hand {
            cards: vec![1, 1, 2, 3, 4],
            bid: 0,
        };
        assert_eq!(h.power(), 1);
    }

    #[test]
    fn test_hand_power_highcard() {
        let h = Hand {
            cards: vec![12, 9, 8, 7, 6],
            bid: 0,
        };
        assert_eq!(h.power(), 0);
    }

    #[test]
    fn test_two_hands_same_power_but_one_is_better() {
        let h1 = Hand {
            cards: vec![12, 12, 4, 5, 5],
            bid: 0,
        };
        let h2 = Hand {
            cards: vec![12, 4, 4, 12, 5],
            bid: 0,
        };
        assert_eq!(h1.cmp(&h2), Ordering::Greater);
    }
}
