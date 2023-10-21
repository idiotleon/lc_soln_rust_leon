/// @author: Leon
/// https://leetcode.com/problems/squares-of-a-sorted-array/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1) / O(`len_ns`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let len_ns: usize = nums.len();
        let mut lo: usize = 0;
        let mut hi: usize = len_ns - 1;
        let mut ans: Vec<i32> = vec![0; len_ns];
        // with `usize` here, all test cases still pass,
        // but `idx` ended up with usize overflow
        let mut idx: isize = len_ns as isize - 1;
        while idx >= 0 {
            if nums[lo].abs() > nums[hi].abs() {
                ans[idx as usize] = nums[lo] * nums[lo];
                lo += 1;
            } else {
                ans[idx as usize] = nums[hi] * nums[hi];
                hi -= 1;
            }
            idx -= 1;
        }
        ans
    }
}
