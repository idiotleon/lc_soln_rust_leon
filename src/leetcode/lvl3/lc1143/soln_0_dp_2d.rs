// @author: Leon
// https://leetcode.com/problems/longest-common-subsequence/
//
// Time Complexity:     O(`len1` * `len2`)
// Space Complexity:    O(`len1` * `len2`)
use std::cmp::max;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {       
        let len1 = text1.chars().count();
        let len2 = text2.chars().count();
        
        let mut dp = vec![vec![0; 1 + len2]; 1 + len1];
        
        for (idx1, ch1) in text1.chars().enumerate(){
            for (idx2, ch2) in text2.chars().enumerate(){
                
                dp[1 + idx1][1 + idx2] = if ch1 == ch2 {
                    1 + dp[idx1][idx2]
                }else{
                    max(dp[1 + idx1][idx2], dp[idx1][1 + idx2])
                }
            }
        }
        
        dp[len1][len2]
    }
}