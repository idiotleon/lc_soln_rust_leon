use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/two-sum/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`len_ns`)
/// Reference:
/// https://leetcode.com/problems/two-sum/discuss/715951/Rust%3A-HashMap-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let len_ns = nums.len();
        let mut num_to_idx: HashMap<i32, usize> = HashMap::with_capacity(len_ns);
        for (idx, num) in nums.into_iter().enumerate() {
            let expected_sum = target - num;
            match num_to_idx.get(&expected_sum) {
                Some(&prev_idx) => {
                    return vec![prev_idx as i32, idx as i32];
                }
                None => {
                    num_to_idx.insert(num, idx);
                }
            }
        }
        unreachable!()
    }
}
