use crate::Solution;

use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn min_cost(max_time: i32, edges: Vec<Vec<i32>>, passing_fees: Vec<i32>) -> i32 {
        // always gonna be (cost, time) and maybe (to, (cost, time))
        let mut map: HashMap<i32, Vec<(i32, (i32, i32))>> = Default::default();

        for e in edges {
            let (from, to, time) = (e[0], e[1], e[2]);

            map.entry(from)
                .or_default()
                .push((to, (passing_fees[to as usize], time)));

            map.entry(to)
                .or_default()
                .push((from, (passing_fees[from as usize], time)));
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        struct Location {
            location: i32,
            cost: i32,
            time: i32,
        }
        impl Ord for Location {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.cost.cmp(&other.cost).then(self.time.cmp(&other.time))
            }
        }
        impl PartialOrd for Location {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut heap: BinaryHeap<Location> = Default::default();
        heap.push(Location {
            location: 0,
            cost: passing_fees[0],
            time: 0,
        });

        let mut min_cost = i32::MAX;
        let goal = passing_fees.len() as i32 - 1;
        let mut best = vec![(i32::MAX, i32::MAX); passing_fees.len()];

        while let Some(Location {
            location,
            cost,
            time,
        }) = heap.pop()
        {
            if !(time <= max_time && cost < min_cost) {
                continue;
            }

            let (best_cost, best_time) = &mut best[location as usize];

            if location == goal {
                min_cost = min_cost.min(cost);
                continue;
            }

            if cost > *best_cost && time > *best_time {
                continue;
            }

            if time < *best_time {
                *best_cost = cost;
                *best_time = time;
            }

            for (to, (extra_cost, extra_time)) in &map[&location] {
                if time + extra_time <= max_time && cost + extra_cost < min_cost {
                    heap.push(Location {
                        location: *to,
                        cost: cost + extra_cost,
                        time: time + extra_time,
                    })
                }
            }
        }

        if min_cost == i32::MAX {
            -1
        } else {
            min_cost
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;
    #[test]
    fn test_1() {
        let ans = Solution::min_cost(
            30,
            vec![
                vec![0, 1, 10],
                vec![1, 2, 10],
                vec![2, 5, 10],
                vec![0, 3, 1],
                vec![3, 4, 10],
                vec![4, 5, 15],
            ],
            vec![5, 1, 2, 20, 20, 3],
        );

        assert_eq!(11, ans);
    }

    #[test]
    fn test_2() {
        let ans = Solution::min_cost(
            29,
            vec![
                vec![0, 1, 10],
                vec![1, 2, 10],
                vec![2, 5, 10],
                vec![0, 3, 1],
                vec![3, 4, 10],
                vec![4, 5, 15],
            ],
            vec![5, 1, 2, 20, 20, 3],
        );

        assert_eq!(48, ans);
    }

    #[test]
    fn test_3() {
        let ans = Solution::min_cost(
            25,
            vec![
                vec![0, 1, 10],
                vec![1, 2, 10],
                vec![2, 5, 10],
                vec![0, 3, 1],
                vec![3, 4, 10],
                vec![4, 5, 15],
            ],
            vec![5, 1, 2, 20, 20, 3],
        );

        assert_eq!(-1, ans);
    }

    #[test]
    fn test_4() {
        let edges: Vec<Vec<_>> = [
            [9, 7, 18],
            [26, 3, 12],
            [28, 45, 33],
            [47, 10, 27],
            [34, 18, 38],
            [32, 13, 39],
            [32, 26, 32],
            [12, 0, 2],
            [4, 1, 7],
            [5, 3, 2],
            [39, 25, 27],
            [45, 10, 34],
            [3, 19, 5],
            [25, 32, 23],
            [30, 10, 47],
            [37, 2, 31],
            [10, 32, 15],
            [23, 14, 19],
            [22, 6, 14],
            [45, 39, 38],
            [39, 21, 30],
            [42, 17, 42],
            [20, 17, 15],
            [24, 0, 27],
            [2, 46, 11],
            [2, 24, 13],
            [36, 22, 30],
            [2, 1, 31],
            [41, 35, 45],
            [4, 19, 20],
            [32, 27, 33],
            [38, 46, 1],
            [21, 11, 15],
            [33, 41, 2],
            [45, 18, 30],
            [8, 33, 50],
            [37, 11, 6],
            [25, 17, 42],
            [45, 39, 33],
            [7, 4, 49],
            [17, 42, 36],
            [36, 16, 9],
            [46, 25, 24],
            [43, 4, 6],
            [35, 13, 28],
            [1, 28, 1],
            [34, 35, 15],
            [38, 1, 15],
            [16, 6, 28],
            [13, 0, 42],
            [3, 30, 24],
            [43, 27, 35],
            [8, 0, 45],
            [27, 20, 47],
            [6, 16, 47],
            [0, 34, 35],
            [0, 35, 3],
            [40, 11, 24],
            [1, 0, 49],
            [44, 20, 32],
            [26, 12, 17],
            [3, 2, 25],
            [37, 25, 42],
            [27, 1, 15],
            [36, 25, 38],
            [24, 47, 33],
            [33, 28, 15],
            [25, 43, 37],
            [47, 31, 47],
            [29, 10, 50],
            [11, 1, 21],
            [29, 3, 48],
            [1, 25, 10],
            [48, 17, 16],
            [19, 24, 22],
            [30, 7, 2],
            [11, 22, 19],
            [20, 42, 41],
            [27, 3, 48],
            [17, 0, 34],
            [19, 14, 32],
            [49, 2, 20],
            [10, 3, 38],
            [0, 49, 13],
            [6, 3, 28],
            [42, 23, 6],
            [14, 8, 1],
            [35, 16, 3],
            [17, 7, 40],
            [18, 7, 49],
            [36, 35, 13],
            [14, 40, 45],
            [16, 33, 11],
            [31, 22, 33],
            [38, 15, 48],
            [15, 14, 25],
            [37, 13, 37],
            [44, 32, 7],
            [48, 1, 31],
            [33, 12, 20],
            [22, 26, 23],
            [4, 10, 11],
            [43, 28, 43],
            [19, 8, 14],
            [35, 31, 33],
            [28, 27, 19],
            [40, 11, 36],
            [36, 43, 28],
            [22, 21, 15],
        ]
        .map(|s| s.to_vec())
        .to_vec();
        let passing_fees = vec![
            199, 505, 107, 961, 682, 400, 304, 517, 512, 18, 334, 627, 893, 412, 922, 289, 19, 161,
            206, 879, 336, 831, 577, 802, 139, 348, 440, 219, 273, 691, 99, 858, 389, 955, 561,
            353, 937, 904, 858, 704, 548, 497, 787, 546, 241, 67, 743, 42, 87, 137,
        ];
        let ans = Solution::min_cost(500, edges, passing_fees);
        assert_eq!(336, ans);
    }

    #[test]
    fn test_5() {
        let edges: Vec<Vec<_>> = [
            [2, 9, 14],
            [1, 8, 25],
            [6, 10, 1],
            [8, 0, 4],
            [0, 4, 12],
            [7, 11, 30],
            [10, 3, 26],
            [9, 8, 8],
            [3, 10, 23],
            [11, 5, 19],
            [4, 0, 4],
            [5, 4, 12],
            [7, 3, 19],
            [10, 9, 5],
            [1, 10, 22],
            [0, 2, 6],
            [9, 4, 15],
            [10, 5, 25],
            [9, 11, 10],
            [9, 1, 21],
            [9, 6, 19],
            [10, 8, 28],
        ]
        .map(|s| s.to_vec())
        .to_vec();
        let passing_fees = vec![24, 12, 24, 30, 18, 20, 18, 30, 28, 10, 6, 7];
        let ans = Solution::min_cost(30, edges, passing_fees);
        assert_eq!(59, ans);
    }

    #[test]
    fn test_6() {
        let edges: Vec<Vec<_>> = [
            [1, 2, 4],
            [2, 0, 9],
            [3, 0, 4],
            [0, 1, 9],
            [4, 3, 10],
            [4, 3, 8],
            [3, 1, 2],
            [2, 1, 9],
            [0, 2, 1],
            [4, 1, 2],
        ]
        .map(|s| s.to_vec())
        .to_vec();
        let passing_fees = vec![8, 5, 10, 4, 10];
        let ans = Solution::min_cost(10, edges, passing_fees);
        assert_eq!(27, ans);
        assert!(false);
    }
}
