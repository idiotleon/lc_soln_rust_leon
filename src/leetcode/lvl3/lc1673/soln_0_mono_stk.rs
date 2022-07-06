use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/find-the-most-competitive-subsequence/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`len_ns`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let len_ns: usize = nums.len();
        let limit: u32 = len_ns as u32 - k as u32;
        let mut stk: VecDeque<i32> = VecDeque::with_capacity(len_ns);
        let mut cnt: u32 = 0;
        for num in nums {
            while !stk.is_empty() && cnt < limit && *stk.back().unwrap() > num {
                stk.pop_back();
                cnt += 1;
            }
            stk.push_back(num);
        }
        while stk.len() > k as usize {
            stk.pop_back();
        }
        stk.into_iter().collect()
    }
}
