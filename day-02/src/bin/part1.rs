use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take_until, take_while1},
    character::{
        self,
        complete::{alpha1, anychar, digit1, line_ending, one_of},
    },
    combinator::{map_res, opt},
    complete::tag,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated, tuple},
    IResult,
};

fn main() {
    let input = include_str!("./input1.txt");
    part1(input);
}

fn part1(input: &str) -> String {}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = map_res(take_while1(|c: char| c.is_digit(10)), |s: &str| {
        s.parse::<usize>()
    })(input)?;

    let (input, sets) =
        terminated(separated_list1(tag(";"), many1(anychar)), opt(tag(";")))(input)?;
    let sets = sets
        .iter()
        .map(|chars: &Vec<char>| {
            let (_, res) = parse_set(&chars.into_iter().collect::<String>()).unwrap();
            res
        })
        .collect();

    Ok((input, Game { id, sets }))
}

fn parse_set(input: &str) -> IResult<&str, HashMap<Color, usize>> {
    map_res(
        terminated(separated_list1(tag(","), many1(anychar)), opt(tag(","))),
        |chars: Vec<_>| {
            println!("{chars:?}");
            let (input, (count, color_string)) = tuple((digit1, alpha1))(chars).unwrap();
            let color = match color_string.as_str() {
                "red" => Color::RED,
                "green" => Color::GREEN,
                "blue" => Color::BLUE,
                _ => unreachable!(),
            };
            (color, count.parse::<usize>().unwrap())
        },
    )(input)
}

struct Game {
    id: usize,
    sets: Vec<HashMap<Color, usize>>,
}

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
}
