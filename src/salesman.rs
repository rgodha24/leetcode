use crate::Solution;

impl Solution {
    pub fn maximize_the_profit(n: i32, offers: Vec<Vec<i32>>) -> i32 {
        let mut ans = vec![0; n as usize];

        for offer in offers.into_iter() {
            for house in offer[0]..offer[1] {
                ans[house as usize] = ans[house as usize].max(offer[2]);
            }

        }
        

        ans.into_iter().sum()
    }
}
