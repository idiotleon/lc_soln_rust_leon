/// @author: Leon
/// https://leetcode.com/problems/longest-continuous-increasing-subsequence/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let len_ns: usize = nums.len();
        let mut idx: usize = 0;
        let mut longest: i32 = 0;
        let mut len: i32 = 1;
        while idx < len_ns {
            while idx < len_ns - 1 && nums[idx] < nums[idx + 1] {
                len += 1;
                idx += 1;
            }
            longest = std::cmp::max(longest, len);
            len = 1;
            idx += 1;
        }
        return longest;
    }
}
