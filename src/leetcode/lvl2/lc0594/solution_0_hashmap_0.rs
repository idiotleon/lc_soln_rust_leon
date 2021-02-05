/**
 * https://leetcode.com/problems/longest-harmonious-subsequence/
 *
 * Time Complexity:     O()
 * Space Complexity:    O()
 *
 * Reference:
 *  https://leetcode.com/problems/longest-harmonious-subsequence/discuss/257536/simple-rust/370259
 */
use std::cmp::max;
use std::collections::HashMap;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut num_to_freq = HashMap::new();
        for num in nums {
            let entry = num_to_freq.entry(num).or_insert(0);
            *entry += 1;
        }
        let mut longest = 0;
        for (num, freq1) in &num_to_freq {
            if let Some(freq2) = num_to_freq.get(&(num + 1)) {
                longest = max(longest, freq1 + freq2)
            }
        }
        longest
    }
}
