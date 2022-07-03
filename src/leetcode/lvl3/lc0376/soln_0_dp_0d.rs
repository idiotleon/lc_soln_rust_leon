/// @author: Leon
/// https://leetcode.com/problems/wiggle-subsequence/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn wiggle_max_length(nums: Vec<i32>) -> i32 {
        let len_n: usize = nums.len();
        let mut cnt_up: i32 = 1;
        let mut cnt_down: i32 = 1;
        for idx in 1..len_n {
            if nums[idx - 1] > nums[idx] {
                cnt_down = cnt_up + 1;
            } else if nums[idx - 1] < nums[idx] {
                cnt_up = cnt_down + 1;
            }
        }
        std::cmp::max(cnt_up, cnt_down)
    }
}
