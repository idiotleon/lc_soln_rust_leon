/// @author: Leon
/// https://leetcode.com/problems/number-of-ways-to-reach-a-position-after-exactly-k-steps/
/// Time Complexity:    O(`RANGE`)
/// Space Complexity:   O(`RANGE`)
/// Reference:
/// https://leetcode.com/problems/number-of-ways-to-reach-a-position-after-exactly-k-steps/discuss/2527513/Simple-DP-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn number_of_ways(start_pos: i32, end_pos: i32, k: i32) -> i32 {
        const RANGE: usize = 1e3 as usize + 1;
        const MOD: i32 = 1e9 as i32 + 7;
        let mut dp: Vec<i32> = vec![0; 3 * RANGE + 1];
        let start_pos = start_pos as usize + RANGE;
        let end_pos = end_pos as usize + RANGE;
        dp[start_pos] = 1;
        for _ in 0..k {
            let mut prev = 0;
            for idx in 0..3 * RANGE {
                let cur = dp[idx];
                dp[idx] = (prev + dp[idx + 1]) % MOD;
                prev = cur;
            }
        }
        return dp[end_pos];
    }
}
