use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    combinator::{map_res, recognize},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

fn main() {
    let input = include_str!("./input1.txt");
    println!("{}", part1(input));
}

fn part1(input: &str) -> String {
    let mut set_limits = HashMap::new();
    set_limits.insert(Color::Red, 12);
    set_limits.insert(Color::Green, 13);
    set_limits.insert(Color::Blue, 14);
    input
        .lines()
        .map(|line| match parse_game(line) {
            Ok((_, game)) => game.set.values().product(),
            Err(_) => 1,
        })
        .sum::<usize>()
        .to_string()
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, id) = delimited(
        tag("Game "),
        map_res(recognize(digit1), str::parse::<usize>),
        tag(": "),
    )(input)?;

    let (input, sets) = separated_list1(tag("; "), parse_set)(input)?;

    let mut map: HashMap<Color, usize> = HashMap::new();
    for set in sets.into_iter() {
        for handful in set.into_iter() {
            match map.get(&handful.0) {
                Some(&value) => map.insert(handful.0, value.max(handful.1)),
                None => map.insert(handful.0, handful.1),
            };
        }
    }

    let game = Game { id, set: map };
    Ok((input, game))
}

fn parse_set(input: &str) -> IResult<&str, Vec<(Color, usize)>> {
    let (input, handfuls) = separated_list1(tag(", "), parse_handful)(input)?;
    Ok((input, handfuls))
}

fn parse_handful(input: &str) -> IResult<&str, (Color, usize)> {
    let (input, (count, color_str)) = separated_pair(
        map_res(recognize(digit1), str::parse::<usize>),
        tag(" "),
        alpha1,
    )(input)?;

    let color = match color_str {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        _ => unreachable!(),
    };
    Ok((input, (color, count)))
}

#[derive(Debug, PartialEq)]
struct Game {
    id: usize,
    set: HashMap<Color, usize>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!("2286", part1(input));
    }

    #[test]
    fn test_parse_handful() {
        let input = "10 red";
        let (_, (color, count)) = parse_handful(input).unwrap();
        assert_eq!(color, Color::Red);
        assert_eq!(count, 10);
        let input = "2 blue";
        let (_, (color, count)) = parse_handful(input).unwrap();
        assert_eq!(color, Color::Blue);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_parse_set() {
        let input = "10 red, 2 green";
        let (_, res) = parse_set(input).unwrap();
        assert_eq!(res, vec![(Color::Red, 10), (Color::Green, 2)]);

        let input = "4 blue";
        let (_, res) = parse_set(input).unwrap();
        assert_eq!(res, vec![(Color::Blue, 4)]);
    }

    #[test]
    fn test_parse_game() {
        let input = "Game 2: 10 red, 2 green; 2 blue";
        let (_, res) = parse_game(input).unwrap();
        let mut map = HashMap::new();
        map.insert(Color::Red, 10);
        map.insert(Color::Green, 2);
        map.insert(Color::Blue, 2);
        assert_eq!(res, Game { id: 2, set: map });
    }
}
