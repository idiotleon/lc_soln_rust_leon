use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/maximum-of-minimum-values-in-all-subarrays/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`len_ns`)
/// Reference:
/// https://leetcode.com/problems/maximum-of-minimum-values-in-all-subarrays/discuss/1310252/Java-O(n)-using-Stacks
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_maximums(nums: Vec<i32>) -> Vec<i32> {
        let len_ns = nums.len();
        let mut stk: VecDeque<usize> = VecDeque::with_capacity(len_ns);
        let right: Vec<i32> = {
            let mut tmp: Vec<i32> = vec![len_ns as i32; len_ns];
            for idx in 0..len_ns {
                Self::process(&mut stk, &mut tmp, &nums, idx);
            }
            tmp
        };
        stk.clear();
        let left: Vec<i32> = {
            let mut tmp: Vec<i32> = vec![-1; len_ns];
            for idx in (0..len_ns).rev() {
                Self::process(&mut stk, &mut tmp, &nums, idx);
            }
            tmp
        };
        let mut ans: Vec<i32> = vec![0; len_ns];
        for idx in 0..len_ns {
            let len = right[idx] - left[idx] - 1;
            ans[len as usize - 1] = std::cmp::max(ans[len as usize - 1], nums[idx]);
        }
        for idx in (0..len_ns - 1).rev() {
            ans[idx] = std::cmp::max(ans[idx], ans[idx + 1]);
        }
        return ans;
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
