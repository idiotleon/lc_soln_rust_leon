/// @author: Leon
/// https://leetcode.com/problems/fixed-point/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn fixed_point(nums: Vec<i32>) -> i32 {
        let _len_n: usize = nums.len();
        for (idx, num) in nums.into_iter().enumerate() {
            if num >= 0 {
                if num == idx as i32 {
                    return idx as i32;
                }
            }
        }
        -1
    }
}
