/// @author: Leon
/// https://leetcode.com/problems/interleaving-string/
/// Time Complexity:    O(`len1` * `len2`)
/// Space Complexity:   O(`len1` * `len2`)
/// Reference:
/// https://leetcode.com/problems/interleaving-string/discuss/31879/My-DP-solution-in-C++/30687
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let len1: usize = s1.len();
        let len2: usize = s2.len();
        let len3: usize = s3.len();
        let chs1: Vec<char> = s1.chars().collect();
        let chs2: Vec<char> = s2.chars().collect();
        let chs3: Vec<char> = s3.chars().collect();
        if len1 + len2 != len3 {
            return false;
        }
        let mut dp: Vec<Vec<bool>> = {
            let mut dp: Vec<Vec<bool>> = vec![vec![false; len2 + 1]; len1 + 1];
            dp[0][0] = true;
            for idx1 in 1..=len1 {
                dp[idx1][0] = dp[idx1 - 1][0] && chs1[idx1 - 1] == chs3[idx1 - 1];
            }
            for idx2 in 1..=len2 {
                dp[0][idx2] = dp[0][idx2 - 1] && chs2[idx2 - 1] == chs3[idx2 - 1];
            }
            dp
        };
        for idx1 in 1..=len1 {
            for idx2 in 1..=len2 {
                let idx3 = idx1 + idx2 - 1;
                dp[idx1][idx2] = (dp[idx1 - 1][idx2] && chs1[idx1 - 1] == chs3[idx3])
                    || (dp[idx1][idx2 - 1] && chs2[idx2 - 1] == chs3[idx3]);
            }
        }
        dp[len1][len2]
    }
}
