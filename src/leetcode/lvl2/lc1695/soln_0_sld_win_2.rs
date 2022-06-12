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
        let mut most: i32 = 0;
        let mut sum: i32 = 0;
        for hi in 0..len_n {
            sum += nums[hi];
            while lo < hi && seen.contains(&nums[hi]) {
                sum -= nums[lo];
                seen.remove(&nums[lo]);
                lo += 1;
            }
            seen.insert(nums[hi]);
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
