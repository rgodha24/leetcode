mod sudoku;
mod sudoku_solver;
mod rectangles;
mod max_average_ratio;
mod pivot_integer;
mod salesman;

pub struct Solution;

use core::panic;
use std::collections::{BinaryHeap, HashMap, HashSet};

fn main() {
    println!("Hello, world!");
}

impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        flowerbed.push(0);
        flowerbed.push(1);
        let mut prev_one = -1 as i32;
        let mut count = 0;
        flowerbed.into_iter().enumerate().for_each(|(i, val)| {
            match val {
                0 => {
                    if i == 0 {
                        prev_one = -2;
                    }
                    // it's a 0, so keep on counting...
                }
                1 => {
                    // we found a one

                    // handle the first case
                    if prev_one != -1 {
                        let length = i as i32 - prev_one;
                        print!("{}: {}   ", length, (length as i32 - 2) / 2);
                        prev_one = i as i32;
                        count += (length as i32 - 2) / 2;
                    } else {
                        prev_one = i as i32;
                    }
                }
                _ => panic!("not 0 or 1"),
            };
        });

        n <= count
    }

    pub fn strong_password_checker_ii(password: String) -> bool {
        // 8+ long
        if password.len() < 8 {
            return false;
        };

        let mut last_val = String::from("");
        let mut digit_count = 0;
        let mut uppercase_count = 0;
        let mut lowercase_count = 0;
        let mut special_char_count = 0;
        let mut repeated_char_count = 0;
        for char in password.chars() {
            if !last_val.is_empty() {
                if char == last_val.as_bytes()[0].into() {
                    repeated_char_count += 1;
                }
            }
            if char.is_uppercase() {
                uppercase_count += 1;
            }
            if char.is_lowercase() {
                lowercase_count += 1;
            }
            if char.is_digit(10) {
                digit_count += 1;
            }
            if String::from("!@#$%^&*()-+").contains(char) {
                special_char_count += 1;
            }

            last_val = String::from(char);
        }

        digit_count >= 1
            && uppercase_count >= 1
            && lowercase_count >= 1
            && special_char_count >= 1
            && repeated_char_count == 0
    }
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let roads: Vec<_> = roads.iter().map(|r| (r[0], r[1], r[2])).collect();

        // map from, to, distance
        let mut neighbors: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut distances: HashMap<(i32, i32), usize> = HashMap::new();

        for (a, b, distance) in roads {
            let mut a_neighbors = neighbors.get(&a).unwrap_or(&Default::default()).to_owned();
            let mut b_neighbors = neighbors.get(&b).unwrap_or(&Default::default()).to_owned();

            a_neighbors.push(b);
            b_neighbors.push(a);

            neighbors.insert(a, a_neighbors);
            neighbors.insert(b, b_neighbors);
            distances.insert((a, b), distance as usize);
            distances.insert((b, a), distance as usize);
        }

        // literally just a* because im not smart enough to do it another way (copied from wikipedia pseudocode)

        println!("{:?}", distances.get(&(1, 4)));

        let mut open_set: Vec<i32> = vec![1];
        let mut came_from: HashMap<i32, i32> = HashMap::new();
        let mut g_score: HashMap<i32, usize> = Default::default();

        while !open_set.is_empty() {
            println!("{:?}", open_set);
            let current = open_set.pop().unwrap();

            let current_neighbours = neighbors.get(&current).unwrap();

            println!("{:?}", current_neighbours);

            for neighbor in current_neighbours {
                let tentative_gscore = std::cmp::min(
                    g_score.get(&current).unwrap_or(&1283765),
                    distances.get(&(current, *neighbor)).unwrap(),
                );

                println!("{tentative_gscore}");
                if tentative_gscore < g_score.get(&neighbor).unwrap_or(&std::usize::MAX) {
                    came_from.insert(*neighbor, current);
                    g_score.insert(*neighbor, *tentative_gscore);

                    if !open_set.contains(neighbor) {
                        open_set.push(*neighbor);
                        println!("{:?}", open_set);
                    }
                }
            }
        }

        return Self::path_distance(came_from, distances, n);
    }

    fn path_distance(
        came_from: HashMap<i32, i32>,
        distances: HashMap<(i32, i32), usize>,
        mut current: i32,
    ) -> i32 {
        let keys = came_from.keys().collect::<HashSet<_>>();
        let mut path = Vec::new();

        while keys.contains(&current) {
            let old = current;
            current = came_from[&current];
            path.push((old, current));
        }

        let mut distance = Vec::new();
        for part in path {
            distance.push(*distances.get(&part).unwrap());
        }

        println!("{:?}", distance);

        *distance.iter().min().unwrap() as i32
    }
}
