use crate::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let board: Vec<Vec<Member>> = board
            .into_iter()
            .map(|v| v.into_iter().map(Into::into).collect())
            .collect();

        // check lines
        for row in &board {
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
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Member {
    Empty,
    Contained(Char),
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
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
