use std::collections::HashMap;

/// https://leetcode.com/problems/contains-duplicate-ii/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(`_len_n`)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let _len_n = nums.len();
        let mut num_to_idx: HashMap<i32, usize> = HashMap::new();
        for (idx, num) in nums.into_iter().enumerate() {
            if let Some(&prev_idx) = num_to_idx.get(&num) {
                if idx - prev_idx <= k as usize {
                    return true;
                }
            }
            num_to_idx.insert(num, idx);
        }
        false
    }
}
