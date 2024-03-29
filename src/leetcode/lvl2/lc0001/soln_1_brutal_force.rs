/// @author: Leon
/// https://leetcode.com/problems/two-sum/
/// Time Complexity:    O(`len_n` ^ 2)
/// Space Complexity:   O(1)
/// References:
/// https://leetcode.com/problems/two-sum/discuss/207679/Rust
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len_n = nums.len();
        for lo in 0..len_n {
            for hi in lo + 1..len_n {
                if nums[lo] + nums[hi] == target {
                    return vec![lo as i32, hi as i32];
                }
            }
        }
        unreachable!()
    }
}
