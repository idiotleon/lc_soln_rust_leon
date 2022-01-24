use std::collections::HashMap;
/// @author: Leon
/// https://leetcode.com/problems/sort-array-by-increasing-frequency/
/// Time Complexity:    O(`_len_n` * lg(`_len_n`))
/// Space Complexity:   O(1) / O(`_len_n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let num_to_freq: HashMap<i8, u8> = {
            let mut num_to_freq: HashMap<i8, u8> = HashMap::new();
            for &num in &nums {
                *num_to_freq.entry(num as i8).or_default() += 1;
            }
            num_to_freq
        };
        let sorted: Vec<i32> = {
            let mut sorted = nums;
            sorted.sort_by(|&a, &b| {
                let a: i8 = a as i8;
                let b: i8 = b as i8;
                let freq_a: u8 = *num_to_freq.get(&a).unwrap();
                let freq_b: u8 = *num_to_freq.get(&b).unwrap();
                if freq_a != freq_b {
                    freq_a.cmp(&freq_b)
                } else {
                    b.cmp(&a)
                }
            });
            sorted
        };
        sorted
    }
}
