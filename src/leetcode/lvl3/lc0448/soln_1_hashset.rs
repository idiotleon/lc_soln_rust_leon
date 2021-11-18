/// https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(`len_n`)
#[allow(dead_code)]
struct Solution;

use std::collections::HashSet;

#[allow(dead_code)]
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let len_n = nums.len();
        let mut nums_set = HashSet::<i32>::new();
        for n in 1..=len_n as i32 {
            nums_set.insert(n);
        }
        for num in nums {
            if nums_set.contains(&num) {
                nums_set.remove(&num);
            }
        }
        nums_set.into_iter().collect()
    }
}
