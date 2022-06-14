/// @author: Leon
/// https://leetcode.com/problems/delete-operation-for-two-strings/
/// Time Complexity:    O(`len1` * `len2`)
/// Space Complexity:   O(`len1` * `len2`)
/// Reference:
/// https://leetcode.com/problems/minimum-size-subarray-sum/discuss/59078/Accepted-clean-Java-O(n)-solution-(two-pointers)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let len1: usize = word1.len();
        let chs1: Vec<char> = word1.chars().collect();
        let len2: usize = word2.len();
        let chs2: Vec<char> = word2.chars().collect();
        let mut dp: Vec<Vec<u16>> = vec![vec![0; len2 + 1]; len1 + 1];
        for (idx1, &ch1) in chs1.iter().enumerate() {
            for (idx2, &ch2) in chs2.iter().enumerate() {
                dp[idx1 + 1][idx2 + 1] = if ch1 == ch2 {
                    1 + dp[idx1][idx2]
                } else {
                    std::cmp::max(dp[idx1 + 1][idx2], dp[idx1][idx2 + 1])
                };
            }
        }
        // (the length of the) longest common subsequence
        let lcs: u16 = dp[len1][len2];
        (len1 + len2) as i32 - 2 * lcs as i32
    }
}
