/**
 * https://leetcode.com/problems/longest-harmonious-subsequence/
 *
 * Time Complexity:     O()
 * Space Complexity:    O()
 *
 * Refernece:
 *  https://leetcode.com/problems/longest-harmonious-subsequence/discuss/257536/simple-rust
 */
use std::cmp::max;
use std::collections::HashMap;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut num_to_freq = HashMap::new();
        for num in nums.iter() {
            match num_to_freq.get(num) {
                Some(freq) => num_to_freq.insert(num, 1 + freq),
                None => num_to_freq.insert(num, 1),
            };
        }
        let mut longest = 0;
        for (num, freq1) in &num_to_freq {
            match &num_to_freq.get(&(*num + 1)) {
                Some(freq2) => {
                    let len = *freq1 + **freq2;
                    longest = max(longest, len);
                }
                None => {}
            }
        }
        longest
    }
}
