use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/count-special-quadruplets/
/// Time Complexity:    O(`len_ns` ^ 2)
/// Space Complexity:   O(`len_ns`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        let len_ns: usize = nums.len();
        let mut cnt: i32 = 0;
        let mut sum_to_freq: HashMap<i32, i32> = HashMap::with_capacity(len_ns);
        sum_to_freq.insert(nums[len_ns - 1] - nums[len_ns - 2], 1);
        for mid in (1..len_ns - 2).rev() {
            for lo in (0..mid).rev() {
                if let Some(&freq) = sum_to_freq.get(&(nums[lo] + nums[mid])) {
                    cnt += freq;
                }
            }
            for hi in (mid + 1..len_ns).rev() {
                *sum_to_freq.entry(nums[hi] - nums[mid]).or_default() += 1;
            }
        }
        return cnt;
    }
}
