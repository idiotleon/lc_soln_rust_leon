use std::collections::HashMap;
/// @author: Leon
/// https://leetcode.com/problems/divide-array-into-equal-pairs/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(`_len_n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let _len_n: usize = nums.len();
        let num_to_freq: HashMap<u16, u16> = {
            let mut map: HashMap<u16, u16> = HashMap::new();
            for num in nums {
                *map.entry(num as u16).or_default() += 1;
            }
            map
        };
        for (_num, freq) in num_to_freq {
            if freq % 2 == 1 {
                return false;
            }
        }
        true
    }
}
