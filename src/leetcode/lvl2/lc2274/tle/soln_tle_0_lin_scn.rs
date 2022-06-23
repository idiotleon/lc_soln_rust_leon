use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/maximum-consecutive-floors-without-special-floors/
/// Time Complexity:    O(`top` - `bottom`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_consecutive(bottom: i32, top: i32, special: Vec<i32>) -> i32 {
        let set: HashSet<i32> = special.into_iter().collect();
        let mut len: i32 = 0;
        let mut longest: i32 = 0;
        for cur in bottom..=top{
            if set.contains(&cur){
                len = 0;
            }else{
                len += 1;
                longest = std::cmp::max(longest, len);
            }
        }
        longest
    }
}