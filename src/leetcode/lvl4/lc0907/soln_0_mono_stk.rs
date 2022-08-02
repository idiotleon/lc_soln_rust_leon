use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/sum-of-subarray-minimums/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`len_ns`)
/// Reference:
/// https://leetcode.com/problems/sum-of-subarray-minimums/discuss/170769/Java-O(n)-monotone-stack-with-DP
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sum_subarray_mins(nums: Vec<i32>) -> i32 {
        const MOD: i32 = 1e9 as i32 + 7;
        let len_ns: usize = nums.len();
        let mut stk: VecDeque<isize> = {
            let mut stk: VecDeque<isize> = VecDeque::with_capacity(len_ns);
            stk.push_back(-1);
            stk
        };
        let mut sum: i32 = 0;
        let mut dp: Vec<i32> = vec![0; len_ns + 1];
        for idx in 0..len_ns {
            while let Some(&top) = stk.back() {
                if top != -1 && nums[idx] <= nums[top as usize] {
                    stk.pop_back();
                } else {
                    break;
                }
            }
            if let Some(&top) = stk.back() {
                dp[idx + 1] = (dp[top as usize + 1] + (idx as i32 - top as i32) * nums[idx]) % MOD;
            }
            stk.push_back(idx as isize);
            sum += dp[idx + 1];
            sum %= MOD;
        }
        sum
    }
}
