use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/maximum-of-minimum-values-in-all-subarrays/
/// Time Complexity:    O(`len_n`)
/// Space Complexity;   O(`len_n`)
/// Reference:
/// https://leetcode.com/problems/maximum-of-minimum-values-in-all-subarrays/discuss/1310252/Java-O(n)-using-Stacks
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_maximums(nums: Vec<i32>) -> Vec<i32> {
        let len_n = nums.len();
        let mut stk: VecDeque<usize> = VecDeque::with_capacity(len_n);
        let right: Vec<i32> = {
            let mut tmp: Vec<i32> = vec![len_n as i32; len_n];
            for idx in 0..len_n {
                Self::process(&mut stk, &mut tmp, &nums, idx);
            }
            tmp
        };
        stk.clear();
        let left: Vec<i32> = {
            let mut tmp: Vec<i32> = vec![-1; len_n];
            for idx in (0..len_n).rev() {
                Self::process(&mut stk, &mut tmp, &nums, idx);
            }
            tmp
        };
        let ans: Vec<i32> = {
            let mut tmp: Vec<i32> = vec![0; len_n];
            for idx in 0..len_n {
                let len = right[idx] - left[idx] - 1;
                tmp[len as usize - 1] = std::cmp::max(tmp[len as usize - 1], nums[idx]);
            }
            for idx in (0..len_n - 1).rev() {
                tmp[idx] = std::cmp::max(tmp[idx], tmp[idx + 1]);
            }
            tmp
        };
        ans
    }

    fn process(stk: &mut VecDeque<usize>, indices: &mut Vec<i32>, nums: &Vec<i32>, idx: usize) {
        while let Some(&idx_top) = stk.back() {
            if nums[idx_top] <= nums[idx] {
                break;
            }
            indices[idx_top] = idx as i32;
            stk.pop_back();
        }
        stk.push_back(idx);
    }
}
