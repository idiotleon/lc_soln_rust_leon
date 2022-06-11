/// @author: Leon
/// https://leetcode.com/problems/partition-array-such-that-maximum-difference-is-k/
/// Time Complexity:    O(`len_n` * lg(`len_n`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn partition_array(nums: Vec<i32>, k: i32) -> i32 {
        let len_n: usize = nums.len();
        if len_n == 1 {
            return 1;
        }
        let sorted: Vec<i32> = {
            let mut nums = nums;
            nums.sort();
            nums
        };
        let mut lo: usize = 0;
        let mut hi: usize = 0;
        let mut cnt: i32 = 0;
        while hi < len_n {
            while hi < len_n && sorted[hi] - sorted[lo] <= k {
                hi += 1;
            }
            cnt += 1;
            lo = hi;
        }
        cnt
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_sample_input_1_should_return_expected() {
        let nums: Vec<i32> = vec![3, 6, 1, 2, 5];
        let k: i32 = 2;
        let actual: i32 = Solution::partition_array(nums, k);
        let expected: i32 = 2;
        assert_eq!(expected, actual);
    }
    #[test]
    pub fn test_sample_input_2_should_return_expected() {
        let nums: Vec<i32> = vec![1, 2, 3];
        let k: i32 = 1;
        let actual: i32 = Solution::partition_array(nums, k);
        let expected: i32 = 2;
        assert_eq!(expected, actual);
    }
    #[test]
    pub fn test_sample_input_3_should_return_expected() {
        let nums: Vec<i32> = vec![2, 2, 4, 5];
        let k: i32 = 0;
        let actual: i32 = Solution::partition_array(nums, k);
        let expected: i32 = 3;
        assert_eq!(expected, actual);
    }
    #[test]
    pub fn test_random_input_1_should_return_expected() {
        let nums: Vec<i32> = vec![1, 5, 7];
        let k: i32 = 1;
        let actual: i32 = Solution::partition_array(nums, k);
        let expected: i32 = 3;
        assert_eq!(expected, actual);
    }
}
