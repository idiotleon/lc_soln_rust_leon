/// @author: Leon
/// https://leetcode.com/problems/max-consecutive-ones-iii/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
/// Note:
/// this is a wrong solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let len_n: usize = nums.len();
        let mut longest: usize = 0;
        let mut cnt = k;
        // the running length
        let mut len: usize = 0;
        let mut hi: usize = 0;
        let mut lo: usize = 0;
        while hi < len_n {
            if nums[hi] == 0 {
                if cnt > 0 {
                    cnt -= 1;
                    len = hi - lo + 1;
                } else {
                    // this is incorrect.
                    // to reset the running length and the counter all at once is not enough.
                    // the counter has to be recycled one by one.
                    len = 0;
                    lo = hi;
                    cnt = k;
                }
            }
            longest = std::cmp::max(longest, len);
            hi += 1;
        }
        longest as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_sampe_input_1_should_return_expected() {
        let nums: Vec<i32> = vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0];
        let k: i32 = 2;
        let actual = Solution::longest_ones(nums, k);
        let expected = 6;
        assert_eq!(expected, actual);
    }
}
