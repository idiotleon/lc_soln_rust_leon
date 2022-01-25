/// @author: Leon
/// https://leetcode.com/problems/remove-element/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let len_n: usize = nums.len();
        let mut lo: usize = 0;
        for hi in 0..len_n {
            if nums[hi] == val {
                continue;
            }
            nums[lo] = nums[hi];
            lo += 1;
        }
        lo as i32
    }
}
