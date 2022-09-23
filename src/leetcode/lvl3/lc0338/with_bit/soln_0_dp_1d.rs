/// @author: Leon
/// https://leetcode.com/problems/counting-bits/
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(1) / O(`n`)
/// Reference:
/// https://leetcode.com/problems/counting-bits/discuss/79539/Three-Line-Java-Solution/84518
/// https://leetcode.com/problems/counting-bits/discuss/79539/Three-Line-Java-Solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut dp: Vec<i32> = vec![0; n as usize + 1];
        for i in 0..=n as usize {
            dp[i] = dp[i >> 1] + (i & 1) as i32;
        }
        return dp;
    }
}
