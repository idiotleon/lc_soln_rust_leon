/// @author: Leon
/// https://leetcode.com/problems/longest-increasing-subsequence/
/// Time Complexity:        O(`len_n` ^ 2)
/// Space Complexity:       O(`len_n`)
/// Reference:
/// https://leetcode.com/problems/longest-increasing-subsequence/discuss/74836/My-easy-to-understand-O(n2)-solution-using-DP-with-video-explanation/144619
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let len_n = nums.len();
        let mut dp: Vec<u16> = vec![1; len_n];
        let mut longest: u16 = 1;
        for hi in 1..len_n {
            for lo in 0..hi {
                if nums[lo] < nums[hi] {
                    dp[hi] = std::cmp::max(dp[hi], dp[lo] + 1)
                }
            }
            longest = std::cmp::max(longest, dp[hi])
        }
        longest as i32
    }
}
