use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

#[derive(Clone, Debug)]
enum Tile {
    Given(u8),
    Solved(u8),
    /// ones are things it can not be
    Options(u16),
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            c if c.is_ascii_digit() => Self::Given(c as u8 - 48),
            '.' => Self::Options(Default::default()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Board([[Tile; 9]; 9]);

impl Board {
    fn gen_options(&mut self) {
        // todo: not this
        self.zero_options();
        for y in 0..9 {
            for x in 0..9 {
                let n = match self.0[y][x] {
                    Tile::Given(n) | Tile::Solved(n) => n,
                    _ => continue,
                };

                let x: u8 = x
                    .try_into()
                    .expect("array of len 9 can't have an index >255");
                let y: u8 = y
                    .try_into()
                    .expect("array of len 9 can't have an index >255");

                let c = (0..9).filter(|i| *i != y).map(|y| (x, y));
                let r = (0..9).filter(|i| *i != x).map(|x| (x, y));
                let b = (0..3)
                    .map(|n| n + 3 * (y / 3))
                    .flat_map(move |y| (0..3).map(move |n| n + 3 * (x / 3)).map(move |x| (x, y)));

                c.chain(r).chain(b).for_each(|p| match self[p] {
                    Tile::Given(_) | Tile::Solved(_) => {}
                    Tile::Options(ref mut opts) => {
                        *opts = *opts | 1 << n - 1;
                    }
                })
            }
        }
    }

    fn zero_options(&mut self) {
        points_iter().for_each(|p| match self[p] {
            Tile::Options(ref mut opts) => {
                *opts = 0;
            }
            _ => {}
        })
    }

    /// assumes options are correctly generated before and after this function is called
    fn solve_recursively(&mut self, depth: usize) -> bool {
        if !self.is_solveable_naive() {
            return false;
        }

        let Some((p, _, o)) = points_iter()
            .filter_map(|p| match self[p] {
                Tile::Options(n) => Some((p, n.count_ones(), n)),
                _ => None,
            })
            .max_by_key(|(_, o, _)| *o)
        else {
            // we solved the sudoko!
            return true;
        };

        for n in 1..=9 {
            if (o & (1 << (n - 1))) != 0 {
                continue;
            }
            self[p] = Tile::Solved(n);
            // todo: this is very inefficient
            self.gen_options();

            println!("{self}");

            if self.solve_recursively(depth + 1) {
                // we solved it later down the tree!
                return true;
            }
            // this path doesn't work, so just keep going
        }

        self[p] = Tile::Options(o);
        self.gen_options();
        false
    }

    fn is_solveable_naive(&self) -> bool {
        points_iter().all(|p| match self[p] {
            Tile::Options(0) => false,
            _ => true,
        })
    }
}

fn points_iter() -> impl Iterator<Item = (u8, u8)> {
    (0..9).flat_map(move |y| (0..9).map(move |x| (x, y)))
}

impl IndexMut<(u8, u8)> for Board {
    fn index_mut(&mut self, (x, y): (u8, u8)) -> &mut Self::Output {
        &mut self.0[y as usize][x as usize]
    }
}
impl Index<(u8, u8)> for Board {
    type Output = Tile;

    fn index(&self, (x, y): (u8, u8)) -> &Self::Output {
        &self.0[y as usize][x as usize]
    }
}

pub fn sudoko(in_board: &mut Vec<Vec<char>>) {
    let mut board = Board(
        in_board
            .iter()
            .map(|v| {
                v.iter()
                    .cloned()
                    .map(Into::into)
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap(),
    );

    board.gen_options();
    println!("{board}");
    assert!(board.solve_recursively(0));

    for (x, y) in points_iter() {
        match board[(x, y)] {
            Tile::Given(n) | Tile::Solved(n) => {
                in_board[y as usize][x as usize] = char::from(n + b'0')
            }
            _ => unreachable!("option tile in completed board"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_one_sudoko() {
        let board = vec![
            ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        let _ans = sudoko(&mut board.into_iter().map(Vec::from).collect());
    }

    #[test]
    fn test_two_sudoko() {
        let board = [
            [".", ".", "9", "7", "4", "8", ".", ".", "."],
            ["7", ".", ".", ".", ".", ".", ".", ".", "."],
            [".", "2", ".", "1", ".", "9", ".", ".", "."],
            [".", ".", "7", ".", ".", ".", "2", "4", "."],
            [".", "6", "4", ".", "1", ".", "5", "9", "."],
            [".", "9", "8", ".", ".", ".", "3", ".", "."],
            [".", ".", ".", "8", ".", "3", ".", "2", "."],
            [".", ".", ".", ".", ".", ".", ".", ".", "6"],
            [".", ".", ".", "2", "7", "5", "9", ".", "."],
        ];

        let mut board = Vec::from(board.map(|c| Vec::from(c.map(|s| s.chars().next().unwrap()))));

        let _ans = sudoko(&mut board);
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use colored::Colorize;

        for y in 0..9 {
            for x in 0..9 {
                match self[(x, y)] {
                    Tile::Given(n) => write!(f, "{}", format!("{n}").green())?,
                    Tile::Solved(n) => write!(f, "{}", format!("{n}").blue())?,
                    Tile::Options(n) => write!(f, "{}", format!("{}", 9 - n.count_ones()).red())?,
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
