/// https://leetcode.com/problems/burst-balloons/
///
/// Time Complexity:    O(`len_n` ^ 2)
/// Space Complexity:   O(`len_n` ^ 2)
///
/// Reference:
/// http://zxi.mytechroad.com/blog/dynamic-programming/leetcode-312-burst-balloons/
/// https://www.youtube.com/watch?v=z3hu2Be92UA
/// https://youtu.be/FLbqgyJ-70I?t=7040
use std::cmp::max;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let len_n: usize = nums.len();
        let len_p: usize = len_n + 2;
        let padded: Vec<i32> = {
            let mut tmp: Vec<i32> = vec![0; len_p];
            tmp[0] = 1;
            tmp[len_n + 1] = 1;
            for (idx, &num) in nums.iter().enumerate() {
                tmp[idx + 1] = num;
            }
            tmp
        };
        let mut dp: Vec<Vec<i32>> = vec![vec![0; len_p]; len_p];
        for len in 1..=len_n {
            for lo in 1..len_p - len {
                let hi = lo + len - 1;
                let mut max_coins = 0;

                for k in lo..=hi {
                    max_coins = max(
                        max_coins,
                        dp[lo][k - 1] + padded[lo - 1] * padded[k] * padded[hi + 1] + dp[k + 1][hi],
                    );
                }
                dp[lo][hi] = max_coins;
            }
        }
        dp[1][len_n]
    }
}
