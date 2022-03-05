/// @author: Leon
/// https://leetcode.com/problems/champagne-tower/
/// Time Complexity:    O(`RANGE` ^ 2)
/// Space Complexity:   O(`RANGE` ^ 2)
/// Reference:
/// https://leetcode.com/problems/champagne-tower/discuss/1817685/C%2B%2B-oror-Easy-To-Understand-oror-Full-Explanation
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        const RANGE: usize = 100 + 1;
        let mut dp: Vec<Vec<f64>> = {
            let mut dp: Vec<Vec<f64>> = vec![vec![0.0; RANGE]; RANGE];
            dp[0][0] = poured as f64;
            dp
        };
        for hi in 0..RANGE - 1 {
            for lo in 0..=hi {
                if dp[hi][lo] >= 1.0 {
                    dp[hi + 1][lo] += (dp[hi][lo] - 1.0) / 2.0;
                    dp[hi + 1][lo + 1] += (dp[hi][lo] - 1.0) / 2.0;
                    dp[hi][lo] = 1.0;
                }
            }
        }
        dp[query_row as usize][query_glass as usize]
    }
}
