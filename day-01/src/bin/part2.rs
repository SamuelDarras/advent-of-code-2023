fn main() {
    let input = include_str!("./input1.txt");
    let result = part2(input);
    println!("{result}");
}

const WORDS_MAP: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn part2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut line = line.to_string();
            while let Some((index, value)) = WORDS_MAP
                .iter()
                .enumerate()
                .filter_map(|(i, word)| line.find(word).map(|index| (index, (i + 1).to_string())))
                .min_by(|(index1, _), (index2, _)| index1.cmp(index2))
            {
                line.replace_range(index..(index + 1), &value);
            }
            let digits = line
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<Vec<char>>();
            let left = digits.first().unwrap();
            let right = digits.last().unwrap();
            let value_string = format!("{left}{right}");

            u32::from_str_radix(&value_string, 10).unwrap()
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(part2(input), "281");
    }
}
