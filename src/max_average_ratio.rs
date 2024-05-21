use crate::Solution;

use std::{cmp::Ordering, collections::BinaryHeap};

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut classes: BinaryHeap<Class> = classes.into_iter().map(Class::from).collect();

        for _ in 0..extra_students {
            let mut class = classes.pop().unwrap();
            class += 1;
            classes.push(class);
        }

        let len = classes.len() as f64;

        classes
            .into_iter()
            .map(|c| c.pass as f64 / c.total as f64)
            .sum::<f64>()
            / len
    }
}

#[derive(PartialEq, Eq)]
struct Class {
    pass: i32,
    total: i32,
}

impl std::ops::Add<i32> for Class {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Self {
            pass: self.pass + rhs,
            total: self.total + rhs,
        }
    }
}

impl std::ops::AddAssign<i32> for Class {
    fn add_assign(&mut self, rhs: i32) {
        self.pass += rhs;
        self.total += rhs;
    }
}

impl From<Vec<i32>> for Class {
    fn from(v: Vec<i32>) -> Self {
        Self {
            pass: v[0],
            total: v[1],
        }
    }
}

impl PartialOrd for Class {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Class {
    fn cmp(&self, other: &Self) -> Ordering {
        let ratio = (self.pass + 1) as f32 / (self.total + 1) as f32
            - (self.pass as f32 / self.total as f32);
        let other_ratio = (other.pass + 1) as f32 / (other.total + 1) as f32
            - (other.pass as f32 / other.total as f32);

        // intentionally reversed
        other_ratio.total_cmp(&ratio)
    }
}
