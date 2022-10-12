/// @author: Leon
/// https://leetcode.com/problems/largest-perimeter-triangle/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
        let len_ns: usize = nums.len();
        let nums = {
            let mut nums = nums;
            nums.sort();
            nums
        };
        for idx in (0..=len_ns - 3).rev() {
            if nums[idx] + nums[idx + 1] > nums[idx + 2] {
                return nums[idx] + nums[idx + 1] + nums[idx + 2];
            }
        }
        return 0;
    }
}
