fn main() {
    let input = include_str!("./input1.txt");
    println!("{}", part1(input));
}

fn look_around(cells: &Vec<Vec<Cell>>, center: (usize, usize)) -> usize {
    let mut number = 1;
    let mut count = 0;
    for i in -1..=1 {
        let mut deactivated = Vec::new();
        for j in -1..=1 {
            let off_x = (center.0 as isize + j) as usize;
            let off_y = (center.1 as isize + i) as usize;

            if deactivated.contains(&off_x) {
                continue;
            }

            let current_cell = &cells[off_y][off_x];
            match current_cell {
                Cell::Number { friends, .. } => {
                    count += 1;
                    if count > 2 {
                        return 0;
                    }
                    let new_number = number_from_friends(friends);
                    number *= new_number;
                    friends.iter().for_each(|(idx, _)| deactivated.push(*idx));
                }
                _ => {}
            }
        }
    }
    if count == 2 {
        number
    } else {
        0
    }
}

fn number_from_friends(friends: &Vec<(usize, usize)>) -> usize {
    friends.iter().fold(0, |acc, (_, v)| acc * 10 + v)
}

fn part1(input: &str) -> String {
    let cells = parse_input(input);

    let mut res = 0;

    for (y, row) in cells.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            match cell {
                Cell::Symbol => {
                    let symbol_value = look_around(&cells, (x, y));
                    res += symbol_value;
                }
                _ => {}
            }
        }
    }

    res.to_string()
}

fn parse_input(input: &str) -> Vec<Vec<Cell>> {
    let mut cells: Vec<Vec<Cell>> = input
        .lines()
        .map(|line| {
            let mut row = Vec::new();
            row.push(Cell::None);
            let mut span = 0;

            for char in line.chars() {
                if char.is_numeric() {
                    let row_len = row.len();
                    let previous = &mut row[row_len - span..row_len];

                    let value = char as usize - '0' as usize;
                    let mut friends: Vec<(usize, usize)> = previous
                        .iter()
                        .enumerate()
                        .map(|(i, p)| match p {
                            Cell::Number { value, friends } => {
                                (row_len - friends.len() + i, *value)
                            }
                            _ => unreachable!(),
                        })
                        .collect();
                    for prev in previous.iter_mut() {
                        prev.add_friend(row_len, value);
                    }
                    friends.push((row.len(), value));

                    row.push(Cell::Number { value, friends });
                    span += 1;
                } else if char == '.' {
                    row.push(Cell::None);
                    span = 0;
                } else {
                    row.push(Cell::Symbol);
                    span = 0;
                }
            }
            row.push(Cell::None);
            row
        })
        .collect();
    cells.insert(0, vec![Cell::None; cells[0].len()]);
    cells.push(vec![Cell::None; cells[0].len()]);
    cells
}

#[derive(Debug, Clone)]
enum Cell {
    Number {
        value: usize,
        friends: Vec<(usize, usize)>,
    },
    Symbol,
    None,
}
impl Cell {
    fn add_friend(&mut self, index: usize, value: usize) {
        match self {
            Cell::Number { friends, .. } => friends.push((index, value)),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";
        let res = part1(input);
        assert_eq!("467835", res);
    }
}
