fn main() {
    let input = include_str!("./input1.txt");
    let result = part1(input);
    println!("{result}");
}

fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
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
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(part1(input), "142");
    }
}
