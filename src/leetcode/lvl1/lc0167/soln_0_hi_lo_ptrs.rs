/// @author: Leon
/// https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len_n: usize = nums.len();
        let mut lo: usize = 0;
        let mut hi: usize = len_n - 1;
        while lo < hi {
            let sum = nums[lo] + nums[hi];
            if sum == target {
                return vec![lo as i32 + 1, hi as i32 + 1];
            }
            if sum > target {
                hi -= 1;
            } else {
                lo += 1;
            }
        }
        unreachable!()
    }
}
