/// @author: Leon
/// https://leetcode.com/problems/solving-questions-with-brainpower/
/// Time Complexity:    O(`len_qs`)
/// Space Complexity:   O(`len_qs`)
/// Reference:
/// https://leetcode.com/problems/solving-questions-with-brainpower/discuss/1692963/DP/1221054
/// https://leetcode.com/problems/solving-questions-with-brainpower/discuss/1692963/DP
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let len_qs: usize = questions.len();
        let mut dp: Vec<i64> = vec![0; len_qs];
        dp[len_qs - 1] = questions[len_qs - 1][0] as i64;
        for idx in (0..len_qs - 1).rev() {
            let idx_nxt = idx + questions[idx][1] as usize + 1;
            let point: i64 = questions[idx][0] as i64
                + if idx_nxt < len_qs {
                    dp[idx_nxt] as i64
                } else {
                    0
                };
            dp[idx] = std::cmp::max(dp[idx + 1], point);
        }
        dp[0]
    }
}
