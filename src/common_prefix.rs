use std::collections::HashSet;

use crate::Solution;

impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut sol = 0;
        let hs: HashSet<_> = arr1.into_iter().flat_map(all_prefixes).collect();

        for a2 in arr2 {
            for p in all_prefixes(a2) {
                if !hs.contains(&p) {
                    continue;
                }
                if p > sol {
                    sol = p;
                    break;
                }

                // already have a just as good or better solution
                if p <= sol {
                    break;
                }
            }
        }

        tens(sol)
    }
}

fn tens(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    (n as f32).log10().floor() as i32 + 1
}

fn all_prefixes(n: i32) -> impl Iterator<Item = i32> {
    (0..=tens(n)).map(move |i| (n / 10_i32.pow(i as u32)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            3,
            Solution::longest_common_prefix(vec![1, 10, 100], vec![1000])
        );
    }
    #[test]
    fn test2() {
        assert_eq!(0, Solution::longest_common_prefix(vec![1, 2, 3], vec![4]));
    }
    #[test]
    fn test3() {
        assert_eq!(
            2,
            Solution::longest_common_prefix(vec![2161, 7400, 4662], vec![7542, 7483, 8628, 3345])
        );
    }
}
