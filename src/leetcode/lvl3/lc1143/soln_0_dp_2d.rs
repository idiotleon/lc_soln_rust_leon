// @author: Leon
// https://leetcode.com/problems/longest-common-subsequence/
//
// Time Complexity:     O(`len1` * `len2`)
// Space Complexity:    O(`len1` * `len2`)
use std::cmp::max;

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let len1 = text1.chars().count();
        let len2 = text2.chars().count();
        
        let chs1: Vec<char> = text1.chars().collect();
        let chs2: Vec<char> = text2.chars().collect();
        
        let mut dp = vec![vec![0; 1 + len2]; 1 + len1];
        
        for idx1 in 0..len1 {
            for idx2 in 0..len2{
                if chs1[idx1] == chs2[idx2]{
                    dp[idx1 + 1][idx2 + 1] = max(dp[idx1 + 1][idx2 + 1], 1 + dp[idx1][idx2]);
                }else{
                    dp[1 + idx1][1 + idx2] = max(dp[1 + idx1][idx2], dp[idx1][1 + idx2]); 
                }
            }
        }
        
        dp[len1][len2]
    }
}