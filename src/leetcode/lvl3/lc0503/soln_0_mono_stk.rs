use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/next-greater-element-ii/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`len_ns`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let len_ns: usize = nums.len();
        let ans: Vec<i32> = {
            let mut res: Vec<i32> = vec![-1; len_ns];
            let mut stk: VecDeque<usize> = VecDeque::with_capacity(len_ns);
            for idx in 0..len_ns * 2 {
                while let Some(&idx_top) = stk.back() {
                    if nums[idx % len_ns] > nums[idx_top % len_ns] {
                        res[stk.pop_back().unwrap() % len_ns] = nums[idx % len_ns];
                    } else {
                        break;
                    }
                }
                stk.push_back(idx);
            }
            res
        };
        return ans;
    }
}
