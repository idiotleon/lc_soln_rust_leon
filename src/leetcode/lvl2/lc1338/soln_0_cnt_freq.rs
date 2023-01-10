use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/reduce-array-size-to-the-half/
/// Time Complexity:    O(`len_ns` * lg(`len_ns`))
/// Space Complexity:   O(`len_ns`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_set_size(nums: Vec<i32>) -> i32 {
        let len_ns: usize = nums.len();
        let num_to_freqs: HashMap<i32, u32> = {
            let mut map: HashMap<i32, u32> = HashMap::with_capacity(len_ns);
            for num in nums {
                *map.entry(num).or_default() += 1;
            }
            map
        };
        let freqs: Vec<u32> = {
            let mut freqs: Vec<u32> = num_to_freqs.into_values().collect();
            freqs.sort_unstable();
            freqs
        };
        let mut cnt: i32 = 0;
        let mut len: u32 = 0;
        let len_ns: u32 = len_ns as u32;
        for freq in freqs.into_iter().rev() {
            len += freq;
            cnt += 1;
            if len >= len_ns / 2 {
                return cnt;
            }
        }
        unreachable!();
    }
}
