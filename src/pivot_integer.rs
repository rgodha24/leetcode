use crate::Solution;

impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        fn sum_between(a: i32, b: i32) -> i32 {
            assert!(a <= b);
            (b - a + 1) * (a + b) / 2
        }

        (1..=n).find(|p| sum_between(1, *p) == sum_between(*p, n)).unwrap_or(-1)
    }
}
