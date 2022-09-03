/// @author: Leon
/// https://leetcode.com/problems/check-if-array-is-sorted-and-rotated/
/// Time Complexity:    O(`len_ns`)
/// Space Compleixty:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let len_ns: usize = nums.len();
        let mut flag: bool = false;
        for idx in 0..len_ns {
            if nums[idx] > nums[(idx + 1) % len_ns] {
                if flag {
                    return false;
                }
                flag = true;
            }
        }
        return true;
    }
}
