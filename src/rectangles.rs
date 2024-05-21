use crate::Solution;

impl Solution {
    pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
        let mut map = std::collections::HashMap::new();
        rectangles.iter().for_each(|v| {
            let ratio = v[0] as f64 / v[1] as f64;
            let count = map.entry(ratio.to_bits()).or_insert(0);
            *count += 1;
        });

        map.values().map(|v| Self::variants(*v)).sum::<i32>() as i64
    }

    fn variants(n: i32) -> i32 {
        n * (n - 1) / 2
    }
}
