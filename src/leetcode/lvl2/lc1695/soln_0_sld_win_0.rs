/// @author: Leon
/// https://leetcode.com/problems/maximum-erasure-value/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        const RANGE: usize = 10e4 as usize + 7;
        let len_n: usize = nums.len();
        let mut freqs: Vec<u16> = vec![0; RANGE];
        let mut lo: usize = 0;
        let mut most: i32 = 0;
        let mut sum: i32 = 0;
        for hi in 0..len_n {
            sum += nums[hi];
            freqs[nums[hi] as usize] += 1;
            while lo < hi && freqs[nums[hi] as usize] > 1 {
                sum -= nums[lo];
                freqs[nums[lo] as usize] -= 1;
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
