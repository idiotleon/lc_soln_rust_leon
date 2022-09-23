/// @author: Leon
/// https://leetcode.com/problems/counting-bits/
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(1) / O(`n`)
/// Reference:
/// https://leetcode.com/problems/counting-bits/discuss/79539/Three-Line-Java-Solution/84518
/// https://leetcode.com/problems/counting-bits/discuss/79539/Three-Line-Java-Solution/84532
/// https://leetcode.com/problems/counting-bits/discuss/79539/Three-Line-Java-Solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut dp: Vec<i32> = vec![0; 1 + n as usize];
        let mut offset: usize = 1;
        for idx in 1..=n as usize {
            if offset * 2 == idx {
                offset *= 2;
            }
            dp[idx] = 1 + dp[idx - offset];
        }
        return dp;
    }
}
