use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/maximum-erasure-value/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/maximum-erasure-value/discuss/978552/Java-O(n)-Sliding-Window-%2B-HashSet
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let len_n: usize = nums.len();
        let mut seen = HashSet::<i32>::new();
        let mut sum: i32 = 0;
        let mut max_sum: i32 = 0;
        let mut lo: usize = 0;
        let mut hi: usize = 0;
        while hi < len_n {
            if seen.insert(nums[hi]) {
                sum += nums[hi];
                max_sum = std::cmp::max(max_sum, sum);
                hi += 1;
            } else {
                sum -= nums[lo];
                seen.remove(&nums[lo]);
                lo += 1;
            }
        }
        max_sum
    }
}
