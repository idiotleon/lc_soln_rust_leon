use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/maximum-erasure-value/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(`len_n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let len_n: usize = nums.len();
        let mut num_to_freq: HashMap<i32, u16> = HashMap::new();
        let mut lo: usize = 0;
        let mut most: i32 = 0;
        let mut sum: i32 = 0;
        for hi in 0..len_n {
            sum += nums[hi];
            *num_to_freq.entry(nums[hi]).or_default() += 1;
            while lo < hi && *num_to_freq.get(&nums[hi]).unwrap() > 1 {
                sum -= nums[lo];
                *num_to_freq.entry(nums[lo]).or_default() -= 1;
                lo += 1;
            }
            most = std::cmp::max(most, sum);
        }
        most
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_sample_input_1_should_return_expected() {
        let nums: Vec<i32> = vec![4, 2, 4, 5, 6];
        let actual = Solution::maximum_unique_subarray(nums);
        let expected = 17;
        assert_eq!(expected, actual);
    }
    #[test]
    pub fn test_sample_input_2_should_return_expected() {
        let nums: Vec<i32> = vec![5, 2, 1, 2, 5, 2, 1, 2, 5];
        let actual = Solution::maximum_unique_subarray(nums);
        let expected = 8;
        assert_eq!(expected, actual);
    }
}
