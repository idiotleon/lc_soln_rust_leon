use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/maximum-number-of-balls-in-a-box/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`len_ns`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
        let len_ns: usize = (high_limit - low_limit + 1) as usize;
        let sum_to_freq: HashMap<i32, i32> = {
            let mut map: HashMap<i32, i32> = HashMap::with_capacity(len_ns);
            for num in low_limit..=high_limit {
                let sum = Self::get_sum(num);
                *map.entry(sum).or_default() += 1;
            }
            map
        };
        return sum_to_freq.into_values().max().unwrap();
    }
    fn get_sum(mut num: i32) -> i32 {
        let mut sum: i32 = 0;
        while num > 0 {
            let digit = num % 10;
            sum += digit;
            num /= 10;
        }
        return sum;
    }
}
