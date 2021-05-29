/// https://leetcode.com/problems/partition-array-for-maximum-sum/
///
/// Time Complexity:    O()
/// Space Complexity:   O()
///
/// Reference:
/// https://leetcode.com/problems/partition-array-for-maximum-sum/discuss/370807/dfs-solution-using-memoization-super-easy-to-understand
#[allow(dead_code)]
struct Solution;

use std::cmp::{max, min};

#[allow(dead_code)]
impl Solution {
    pub fn max_sum_after_partitioning(nums: Vec<i32>, k: i32) -> i32 {
        let len_n: usize = nums.len();
        let mut memo: Vec<i32> = vec![0; len_n];
        Self::dfs(0, k as usize, &nums, &mut memo)
    }

    fn dfs(idx_start: usize, k: usize, nums: &Vec<i32>, mut memo: &mut Vec<i32>) -> i32 {
        let len_n: usize = nums.len();
        if idx_start >= len_n {
            return 0;
        }

        if memo[idx_start] > 0 {
            return memo[idx_start];
        }
        let mut cur_max: i32 = 0;
        let mut max_sum: i32 = 0;
        for idx in idx_start..min(len_n, idx_start + k) {
            cur_max = max(cur_max, nums[idx]);
            max_sum = max(
                max_sum,
                cur_max * (idx - idx_start + 1) as i32 + Self::dfs(idx + 1, k, &nums, &mut memo),
            );
        }
        memo[idx_start] = max_sum;
        max_sum
    }
}
