use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/maximum-score-of-a-good-subarray/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`len_ns`)
/// Reference:
/// https://leetcode.com/problems/maximum-score-of-a-good-subarray/editorial/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let len_ns: usize = nums.len();
        let mut stk: VecDeque<usize> = VecDeque::with_capacity(len_ns);
        /*
         * This is an array:
         * 1. of indices, of the next smaller values, from hi to lo,
         * 2. if there is no next smaller value, the index is kept as -1
         */
        let lo_nxt_smaller_indices: Vec<i32> = {
            let mut lo_nxt_smaller_indices: Vec<i32> = vec![-1; len_ns];
            for idx in (0..len_ns).rev() {
                while let Some(&top) = stk.back() {
                    if nums[idx] < nums[top] {
                        if let Some(top_idx) = stk.pop_back() {
                            lo_nxt_smaller_indices[top_idx] = idx as i32;
                        }
                    } else {
                        break;
                    }
                }
                stk.push_back(idx);
            }
            lo_nxt_smaller_indices
        };
        stk.clear();
        /*
         * This is an array:
         * 1. of indices, of the next smaller values, from lo to hi
         * 2. if there is no next smaller value, the index is kept as `len_ns`
         */
        let hi_nxt_smaller_indices: Vec<i32> = {
            let mut hi_nxt_smaller_indices: Vec<i32> = vec![len_ns as i32; len_ns];
            for idx in 0..len_ns {
                while let Some(&top) = stk.back() {
                    if nums[idx] < nums[top] {
                        if let Some(top_idx) = stk.pop_back() {
                            hi_nxt_smaller_indices[top_idx] = idx as i32;
                        }
                    } else {
                        break;
                    }
                }
                stk.push_back(idx);
            }
            hi_nxt_smaller_indices
        };
        let mut ans: i32 = 0;
        for idx in 0..len_ns {
            if lo_nxt_smaller_indices[idx] < k && hi_nxt_smaller_indices[idx] > k {
                ans = std::cmp::max(
                    ans,
                    nums[idx] * (hi_nxt_smaller_indices[idx] - lo_nxt_smaller_indices[idx] - 1),
                )
            }
        }
        return ans;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let nums: Vec<i32> = vec![1, 4, 3, 7, 4, 5];
        let k: i32 = 3;
        let expected: i32 = 15;
        let actual = Solution::maximum_score(nums, k);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let nums: Vec<i32> = vec![5, 5, 4, 5, 4, 1, 1, 1];
        let k: i32 = 0;
        let expected: i32 = 20;
        let actual = Solution::maximum_score(nums, k);
        assert_eq!(expected, actual);
    }
}
