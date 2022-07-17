use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`len_ns`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let len_ns: usize = nums.len();
        const IMPL: i32 = -1;
        let digit_sum_to_nums: HashMap<i32, Vec<i32>> = {
            let mut map: HashMap<i32, Vec<i32>> = HashMap::with_capacity(len_ns);
            for num in nums {
                map.entry(Self::get_sum_digits(num)).or_default().push(num);
            }
            for (_key, values) in map.iter_mut() {
                values.sort();
            }
            map
        };
        let mut largest: i32 = IMPL;
        for (_digit_sum, nums) in digit_sum_to_nums {
            let len_ns: usize = nums.len();
            if len_ns >= 2 {
                let sum = nums[len_ns - 2] + nums[len_ns - 1];
                largest = std::cmp::max(largest, sum);
            }
        }
        largest
    }
    fn get_sum_digits(num: i32) -> i32 {
        let mut sum: i32 = 0;
        let mut num = num;
        while num > 0 {
            sum += num % 10;
            num /= 10;
        }
        sum
    }
}
