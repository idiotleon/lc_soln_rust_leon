/// @author: Leon
/// https://leetcode.com/problems/check-if-an-array-is-consecutive/
/// Time Complexity:    O(`len_ns` * lg(`len_ns`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_consecutive(nums: Vec<i32>) -> bool {
        let len_ns: usize = nums.len();
        let sorted: Vec<i32> = {
            let mut nums = nums;
            nums.sort();
            nums
        };
        for idx in 0..len_ns - 1 {
            if sorted[idx] + 1 != sorted[idx + 1] {
                return false;
            }
        }
        true
    }
}
