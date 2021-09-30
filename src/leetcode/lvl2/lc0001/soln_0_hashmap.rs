/// @author: Leon
/// https://leetcode.com/problems/two-sum/
///
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(`len_n`)
///
/// this version consumes the original vector
///
/// References:
/// https://leetcode.com/problems/two-sum/discuss/715951/Rust%3A-HashMap-solution
use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // not used
        // let len_n = nums.len();

        let mut sum_to_idx = HashMap::<i32, usize>::new();

        for (idx, num) in nums.into_iter().enumerate() {
            let expected_sum = target - num;

            match sum_to_idx.get(&expected_sum) {
                Some(&prev_idx) => return vec![prev_idx as i32, idx as i32],
                _ => {
                    sum_to_idx.insert(num, idx);
                }
            }
        }

        unreachable!()
    }
}
