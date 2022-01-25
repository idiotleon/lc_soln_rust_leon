/// @author: Leon
/// https://leetcode.com/problems/contains-duplicate/submissions/
/// Time Complexity:    O(`len_n` ^ 2)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let len_n: usize = nums.len();
        for lo in 0..len_n - 1 {
            for hi in lo + 1..len_n {
                if nums[lo] == nums[hi] {
                    return true;
                }
            }
        }
        false
    }
}
