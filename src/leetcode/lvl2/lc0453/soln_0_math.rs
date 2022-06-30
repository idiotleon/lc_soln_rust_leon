/// @author: Leon
/// https://leetcode.com/problems/minimum-moves-to-equal-array-elements/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let len_n: usize = nums.len();
        let sum: i32 = nums.iter().sum::<i32>();
        let min: i32 = *nums.iter().min().unwrap();
        sum - min * (len_n as i32)
    }
}
