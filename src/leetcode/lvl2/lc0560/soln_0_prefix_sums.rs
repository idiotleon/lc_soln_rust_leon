/// @author: Leon
/// https://leetcode.com/problems/subarray-sum-equals-k/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(`_len_n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let _len_n = nums.len();
        let mut sum_to_freq = HashMap::<i32, i32>::new();
        let mut cnt = 0;
        let mut sum = 0;
        for num in nums {
            sum += num;
            if sum == k {
                cnt += 1;
            }
            cnt += sum_to_freq.get(&(sum - k)).unwrap_or(&0);
            *sum_to_freq.entry(sum).or_insert(0) += 1;
        }
        cnt
    }
}
