use std::cmp::max;

// @author: Leon
// https://leetcode.com/problems/longest-common-subsequence/
// Time Complexity:     O(`len1` * `len2`)
// Space Complexity:    O(`len1` * `len2`) + O(max(`len1`, `len2`))
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let len1 = text1.len();
        let len2 = text2.len();
        let mut memo = vec![vec![0; 1 + len2]; 1 + len1];
        let chs1: Vec<char> = text1.chars().collect();
        let chs2: Vec<char> = text2.chars().collect();
        Self::dfs(0, 0, &chs1, &chs2, &mut memo)
    }
    fn dfs(
        idx1: usize,
        idx2: usize,
        chs1: &Vec<char>,
        chs2: &Vec<char>,
        memo: &mut Vec<Vec<i32>>,
    ) -> i32 {
        let len1 = chs1.len();
        let len2 = chs2.len();
        if idx1 >= len1 || idx2 >= len2 {
            return 0;
        }
        if memo[idx1][idx2] > 0 {
            return memo[idx1][idx2];
        }
        let mut longest = 0;
        if chs1[idx1] == chs2[idx2] {
            longest = max(longest, 1 + Self::dfs(1 + idx1, 1 + idx2, chs1, chs2, memo));
        } else {
            longest = max(
                Self::dfs(1 + idx1, idx2, chs1, chs2, memo),
                Self::dfs(idx1, 1 + idx2, chs1, chs2, memo),
            );
        }
        memo[idx1][idx2] = longest;
        longest
    }
}
