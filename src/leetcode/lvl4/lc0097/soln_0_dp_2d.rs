/// https://leetcode.com/problems/interleaving-string/
///
/// Time Complexity:    O(`len1` * `len2`)
/// Space Complexity:   O(`len1` * `len2`)
///
/// Reference:
/// https://leetcode.com/problems/interleaving-string/discuss/31879/My-DP-solution-in-C++/30687
/// https://leetcode.com/problems/interleaving-string/discuss/1247457/Rust-DP-solution
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let len1: usize = s1.len();
        let chs1: Vec<char> = s1.chars().collect();
        let len2: usize = s2.len();
        let chs2: Vec<char> = s2.chars().collect();
        let len3: usize = s3.len();
        let chs3: Vec<char> = s3.chars().collect();
        if len1 + len2 != len3 {
            return false;
        }
        let mut dp: Vec<Vec<bool>> = vec![vec![false; len2 + 1]; len1 + 1];
        for idx1 in 0..=len1 {
            for idx2 in 0..=len2 {
                dp[idx1][idx2] = match (idx1, idx2) {
                    (0, 0) => true,
                    (0, _) => dp[idx1][idx2 - 1] && chs2[idx2 - 1] == chs3[idx2 - 1],
                    (_, 0) => dp[idx1 - 1][idx2] && chs1[idx1 - 1] == chs3[idx1 - 1],
                    _ => {
                        let idx3: usize = idx1 + idx2 - 1;
                        (dp[idx1 - 1][idx2] && chs1[idx1 - 1] == chs3[idx3])
                            || (dp[idx1][idx2 - 1] && chs2[idx2 - 1] == chs3[idx3])
                    }
                }
            }
        }
        dp[len1][len2]
    }
}
