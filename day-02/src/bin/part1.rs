use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::alpha1,
    combinator::map_res,
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

fn main() {
    let input = include_str!("./input1.txt");
    println!("{}", part1(input));
}

fn part1(input: &str) -> String {
    let mut set_limits = HashMap::new();
    set_limits.insert(Color::RED, 12);
    set_limits.insert(Color::GREEN, 13);
    set_limits.insert(Color::BLUE, 14);
    input
        .lines()
        .map(|line| match parse_game(line) {
            Ok((_, game)) => {
                if game
                    .sets
                    .keys()
                    .all(|key| game.sets.get(key).unwrap() <= set_limits.get(key).unwrap())
                {
                    game.id
                } else {
                    0
                }
            }
            Err(_) => 0,
        })
        .sum::<usize>()
        .to_string()
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, (_, id, _)) = tuple((
        tag("Game "),
        map_res(take_while1(|c: char| c.is_digit(10)), |s: &str| {
            s.parse::<usize>()
        }),
        tag(": "),
    ))(input)?;

    let (input, sets) = separated_list1(tag("; "), parse_set)(input)?;
    let mut map = HashMap::new();
    for set in sets {
        for handful in set {
            if map.contains_key(&handful.0) {
                let already: usize = *map.get(&handful.0).unwrap();
                map.insert(handful.0, already.max(handful.1));
            } else {
                map.insert(handful.0, handful.1);
            }
        }
    }

    let game = Game { id, sets: map };
    Ok((input, game))
}

fn parse_set(input: &str) -> IResult<&str, Vec<(Color, usize)>> {
    let (input, handfuls) = separated_list1(tag(", "), parse_handful)(input)?;
    Ok((input, handfuls))
}

fn parse_handful(input: &str) -> IResult<&str, (Color, usize)> {
    let (input, (count, _, color_str)) = tuple((
        map_res(take_while1(|c: char| c.is_digit(10)), |s: &str| {
            s.parse::<usize>()
        }),
        tag(" "),
        alpha1,
    ))(input)?;
    let color = match color_str {
        "red" => Color::RED,
        "green" => Color::GREEN,
        "blue" => Color::BLUE,
        _ => unreachable!(),
    };
    Ok((input, (color, count)))
}

#[derive(Debug, PartialEq)]
struct Game {
    id: usize,
    sets: HashMap<Color, usize>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Color {
    RED,
    GREEN,
    BLUE,
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

        assert_eq!("8", part1(input));
    }

    #[test]
    fn test_parse_handful() {
        let input = "10 red";
        let (_, res) = parse_handful(input).unwrap();
        assert_eq!(res.0, Color::RED);
        assert_eq!(res.1, 10);
        let input = "2 blue";
        let (_, res) = parse_handful(input).unwrap();
        assert_eq!(res.0, Color::BLUE);
        assert_eq!(res.1, 2);
    }

    #[test]
    fn test_parse_set() {
        let input = "10 red, 2 green";
        let (_, res) = parse_set(input).unwrap();
        assert_eq!(res, vec![(Color::RED, 10), (Color::GREEN, 2)]);

        let input = "4 blue";
        let (_, res) = parse_set(input).unwrap();
        assert_eq!(res, vec![(Color::BLUE, 4)]);
    }

    #[test]
    fn test_parse_game() {
        let input = "Game 2: 10 red, 2 green; 2 blue";
        let (_, res) = parse_game(input).unwrap();
        let mut map = HashMap::new();
        map.insert(Color::RED, 10);
        map.insert(Color::GREEN, 2);
        map.insert(Color::BLUE, 2);
        assert_eq!(res, Game { id: 2, sets: map });
    }
}
