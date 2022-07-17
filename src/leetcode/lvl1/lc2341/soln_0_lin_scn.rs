use std::collections::HashMap;

/// @author: Leon
/// Time Complexity:    O(`_len_ns`)
/// Space Complexity:   O(`_len_ns`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let _len_ns: usize = nums.len();
        let num_to_freq: HashMap<i32, i32> = {
            let mut map: HashMap<i32, i32> = HashMap::new();
            for num in nums {
                *map.entry(num).or_default() += 1;
            }
            map
        };
        let mut cnt1: i32 = 0;
        let mut cnt2: i32 = 0;
        for (_num, freq) in num_to_freq {
            cnt1 += freq / 2;
            cnt2 += freq % 2;
        }
        vec![cnt1, cnt2]
    }
}
