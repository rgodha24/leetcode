use crate::Solution;

use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone, Hash)]
struct Board {
    board: [[Member; 9]; 9],
}

impl Solution {
    pub fn solve_sudoku(board_char: &mut Vec<Vec<char>>) {
        let board: [[Member; 9]; 9] = board_char
            .iter()
            .map(|v| {
                v.iter()
                    .cloned()
                    .map(|c| c.into())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let mut board = Board { board };

        board.solve().expect("board has a valid solution");

        *board_char = board
            .board
            .iter()
            .map(|v| v.iter().map(|m| m.into()).collect())
            .collect();
    }
}

impl Board {
    pub fn is_valid(&self) -> bool {
        let board = self.board;

        for row in board.iter() {
            if !Self::check_group(row) {
                return false;
            }
        }

        // check cols
        for i in 0..9 {
            let mut col = Vec::new();
            for j in 0..9 {
                col.push(board[j][i]);
            }

            if !Self::check_group(&col) {
                return false;
            }
        }

        // check grids (3x3)
        for x_n in 0..3 {
            for y_n in 0..3 {
                let mut grid = Vec::new();
                for x in 0..3 {
                    for y in 0..3 {
                        grid.push(board[x_n * 3 + x][y_n * 3 + y]);
                    }
                }

                if !Self::check_group(&grid) {
                    return false;
                }
            }
        }

        return true;
    }

    fn check_group(input: &[Member]) -> bool {
        let mut map: HashMap<Char, usize> = HashMap::new();

        for m in input {
            match m {
                Member::Empty => continue,
                Member::Contained(c) => {
                    let entry = map.entry(*c).or_insert(0);
                    *entry += 1;
                }
            }
        }

        map.values().all(|v| *v <= 1)
    }

    // returns None if no empty cells left (we have finished solving )
    fn random_empty(&self) -> Option<(usize, usize)> {
        for i in 0..9 {
            for j in 0..9 {
                if self.board[i][j] == Member::Empty {
                    return Some((i, j));
                }
            }
        }

        None
    }

    fn solve(&mut self) -> Result<(), ()> {
        // y, x
        let (row, col) = match self.random_empty() {
            Some(coord) => coord,
            // no empty cells left, so we're done solving
            None => return Ok(()),
        };

        for c in Char::VARIANTS.iter() {
            let prev = self.board[row][col];
            self.board[row][col] = Member::Contained(*c);

            // is_valid is cheaper than recursing
            if self.is_valid() {
                if let Ok(_) = self.solve() {
                    return Ok(());
                }
            }

            self.board[row][col] = prev;
        }

        // none of the possible values worked, so we have to backtrack
        Err(())
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Member {
    Empty,
    Contained(Char),
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum Char {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Char {
    const VARIANTS: [Char; 9] = [
        Char::One,
        Char::Two,
        Char::Three,
        Char::Four,
        Char::Five,
        Char::Six,
        Char::Seven,
        Char::Eight,
        Char::Nine,
    ];
}

impl From<char> for Member {
    fn from(c: char) -> Member {
        use Char::*;
        use Member::*;

        match c {
            '.' => Empty,
            '1' => Contained(One),
            '2' => Contained(Two),
            '3' => Contained(Three),
            '4' => Contained(Four),
            '5' => Contained(Five),
            '6' => Contained(Six),
            '7' => Contained(Seven),
            '8' => Contained(Eight),
            '9' => Contained(Nine),
            c => panic!("unknown char {c}"),
        }
    }
}

impl Into<char> for &Member {
    fn into(self) -> char {
        use Char::*;
        use Member::*;

        match self {
            Empty => '.',
            Contained(One) => '1',
            Contained(Two) => '2',
            Contained(Three) => '3',
            Contained(Four) => '4',
            Contained(Five) => '5',
            Contained(Six) => '6',
            Contained(Seven) => '7',
            Contained(Eight) => '8',
            Contained(Nine) => '9',
        }
    }
}
