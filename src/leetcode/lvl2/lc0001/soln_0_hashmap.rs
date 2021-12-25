use std::collections::HashMap;
/// @author: Leon
/// https://leetcode.com/problems/two-sum/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(`_len_n`)
/// this version consumes the original vector
/// Reference:
/// https://leetcode.com/problems/two-sum/discuss/715951/Rust%3A-HashMap-solution
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let _len_n = nums.len();
        let mut num_to_idx = HashMap::<i32, usize>::new();
        for (idx, num) in nums.into_iter().enumerate() {
            let expected_sum = target - num;
            match num_to_idx.get(&expected_sum) {
                Some(&prev_idx) => return vec![prev_idx as i32, idx as i32],
                _ => {
                    num_to_idx.insert(num, idx);
                }
            }
        }
        unreachable!()
    }
}
