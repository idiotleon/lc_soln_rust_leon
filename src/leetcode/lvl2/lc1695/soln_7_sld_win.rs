/// this solution is NOT done

/// @author: Leon
/// https://leetcode.com/problems/maximum-erasure-value/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let len_n: usize = nums.len();
        let mut freqs: Vec<i32> = vec![0; 10];
        let mut lo: usize = 0;
        let mut hi: usize = 0;
        let mut most: i32 = 0;
        let mut sum: i32 = 0;
        while hi < len_n {            let mut sum = 0;
            while hi < len_n && freqs[nums[hi] as usize] == 0 {
                freqs[nums[hi] as usize] += 1;
                sum += nums[hi];
                hi += 1;
            }
            most = std::cmp::max(most, sum);
            freqs[nums[hi] as usize] += 1;
            sum += nums[hi];
            while hi < len_n && lo < hi && freqs[nums[hi] as usize] > 1 {
                freqs[nums[lo] as usize] -= 1;
                sum -= nums[lo];
                lo += 1;
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
}
