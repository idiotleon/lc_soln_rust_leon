/// @author: Leon
/// https://leetcode.com/problems/largest-number-at-least-twice-of-others/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let _len_n: usize = nums.len();
        let mut max1: i32 = 0;
        let mut idx_ans: usize = 0;
        let mut max2: i32 = 0;
        for (idx, num) in nums.into_iter().enumerate() {
            if num > max1 {
                max2 = max1;
                max1 = num;
                idx_ans = idx;
            } else if num > max2 {
                max2 = num;
            }
        }
        if max1 >= 2 * max2 {
            idx_ans as i32
        } else {
            -1
        }
    }
}
