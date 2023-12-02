use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take_till, take_while1},
    character::complete::{alpha1, anychar},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};

fn main() {
    let input = include_str!("./input1.txt");
    part1(input);
}

fn part1(input: &str) -> String {
    let mut lines = input.lines();
    let game = parse_game(lines.next().unwrap());
    println!("{game:?}");
    "0".to_string()
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, (_, id, _)) = tuple((
        tag("Game "),
        map_res(take_while1(|c: char| c.is_digit(10)), |s: &str| {
            s.parse::<usize>()
        }),
        tag(":"),
    ))(input)?;

    let (input, sets) = separated_list1(tag(";"), parse_set)(input)?;

    Ok((input, Game { id, sets }))
}

fn parse_set(input: &str) -> IResult<&str, HashMap<Color, usize>> {
    let (input, _) = map_res(separated_list1(tag(","), alpha1), |chars: Vec<_>| {
        // chars.iter().map(|handful: &str| parse_handful(handful));
        Ok::<(), ()>(())
    })(input)?;
    Ok((input, HashMap::new()))
}

fn parse_handful(input: &str) -> IResult<&str, (Color, usize)> {
    let (input, (count, _, color_str)) = tuple((
        map_res(take_while1(|c: char| c.is_digit(10)), |s: &str| {
            s.parse::<usize>()
        }),
        tag(" "),
        alpha1,
    ))(input)?;
    Ok((input, (Color::RED, count)))
}

#[derive(Debug)]
struct Game {
    id: usize,
    sets: Vec<HashMap<Color, usize>>,
}

#[derive(Debug, PartialEq)]
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
        let res = parse_handful(input).unwrap().1;
        assert_eq!(res.0, Color::RED);
        assert_eq!(res.1, 10);
        let input = "2 blue";
        let res = parse_handful(input).unwrap().1;
        assert_eq!(res.0, Color::BLUE);
        assert_eq!(res.1, 2);
    }
}
