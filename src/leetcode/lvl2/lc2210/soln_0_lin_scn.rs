/// @author: Leon
/// https://leetcode.com/problems/count-hills-and-valleys-in-an-array/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        let len_ns: usize = nums.len();
        let mut last: i32 = nums[0];
        let mut cnt: i32 = 0;
        for idx in 1..len_ns - 1 {
            if (last < nums[idx] && nums[idx] > nums[idx + 1])
                || (last > nums[idx] && nums[idx] < nums[idx + 1])
            {
                cnt += 1;
                last = nums[idx];
            }
        }
        cnt
    }
}
