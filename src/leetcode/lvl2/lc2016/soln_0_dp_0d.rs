/// @author: Leon
/// https://leetcode.com/problems/maximum-difference-between-increasing-elements/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let len_ns: usize = nums.len();
        let mut diff: i32 = -1;
        let mut min: i32 = nums[0];
        for idx in 1..len_ns {
            if nums[idx] > min {
                diff = std::cmp::max(diff, nums[idx] - min);
            }
            min = std::cmp::min(min, nums[idx]);
        }
        return diff;
    }
}
