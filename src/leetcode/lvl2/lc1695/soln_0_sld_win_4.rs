use std::collections::HashSet;
/// @author: Leon
/// https://leetcode.com/problems/maximum-erasure-value/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(`len_n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let len_n: usize = nums.len();
        let mut seen: HashSet<i32> = HashSet::new();
        let mut lo: usize = 0;
        let mut hi: usize = 0;
        let mut most: i32 = 0;
        let mut sum: i32 = 0;
        while hi < len_n {
            while hi < len_n && seen.insert(nums[hi]) {
                sum += nums[hi];
                hi += 1;
            }
            most = std::cmp::max(most, sum);
            while hi < len_n && lo < hi && seen.contains(&nums[hi]) {
                seen.remove(&nums[lo]);
                sum -= nums[lo];
                lo += 1;
            }
            if hi < len_n{
                sum += nums[hi];
                seen.insert(nums[hi]);
            }
            hi += 1;
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
        #[test]
    pub fn test_test_case_2_should_return_expected() {
        let nums: Vec<i32> = vec![10000];
        let actual = Solution::maximum_unique_subarray(nums);
        let expected = 10000;
        assert_eq!(expected, actual);
    }
}
