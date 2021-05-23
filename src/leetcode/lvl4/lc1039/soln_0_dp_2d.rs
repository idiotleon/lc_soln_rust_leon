/// https://leetcode.com/problems/minimum-score-triangulation-of-polygon/
///
/// Time Complexity:    O(`len_n` ^ 3)
/// Space Complexity:   O(`len_n` ^ 2)
///
/// Reference:
/// https://leetcode.com/problems/minimum-score-triangulation-of-polygon/discuss/286705/JavaC%2B%2BPython-DP
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_score_triangulation(nums: Vec<i32>) -> i32 {
        let len_n = nums.len();
        let mut dp = vec![vec![0; len_n]; len_n];

        for len in 2..len_n {
            for lo in 0..len_n - len {
                let hi = lo + len;
                let mut min_score: i32 = std::i32::MAX;
                for k in lo + 1..hi {
                    min_score = std::cmp::min(
                        min_score,
                        dp[lo][k] + dp[k][hi] + nums[lo] * nums[hi] * nums[k],
                    );
                }
                dp[lo][hi] = min_score;
            }
        }
        dp[0][len_n - 1]
    }
}
