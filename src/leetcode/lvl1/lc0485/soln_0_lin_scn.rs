/// @author: Leon
/// https://leetcode.com/problems/max-consecutive-ones/
/// Time Complexity:    O(`_len_ns`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let _len_ns: usize = nums.len();
        let mut longest: i32 = 0;
        let mut len: i32 = 0;
        for num in nums {
            if num == 1 {
                len += 1;
            } else {
                len = 0;
            }
            longest = std::cmp::max(longest, len);
        }
        return longest;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_2_should_return_expected() {
        let nums: Vec<i32> = vec![1, 0, 1, 1, 0, 1];
        let expected: i32 = 2;
        let actual: i32 = Solution::find_max_consecutive_ones(nums);
        assert_eq!(expected, actual);
    }
}
