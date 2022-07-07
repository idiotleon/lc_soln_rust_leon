/// @author: Leon
/// https://leetcode.com/problems/length-of-longest-fibonacci-subsequence/
/// Time Complexity:    O(`len_ns` ^ 2)
/// Space Complexity:   O(`len_ns` ^ 2)
/// Reference:
/// https://leetcode.com/problems/length-of-longest-fibonacci-subsequence/discuss/165330/Java-beat-98-DP-%2B-2Sum
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn len_longest_fib_subseq(nums: Vec<i32>) -> i32 {
        let len_ns: usize = nums.len();
        let mut longest: i32 = 0;
        let mut dp: Vec<Vec<i32>> = vec![vec![0; len_ns]; len_ns];
        for idx in 1..len_ns {
            let mut lo: usize = 0;
            let mut hi: usize = idx - 1;
            while lo < hi {
                let sum = nums[lo] + nums[hi];
                if sum > nums[idx] {
                    hi -= 1;
                } else if sum < nums[idx] {
                    lo += 1;
                } else {
                    dp[hi][idx] = 1 + dp[lo][hi];
                    longest = std::cmp::max(longest, dp[hi][idx]);
                    hi -= 1;
                    lo += 1;
                }
            }
        }
        if longest == 0 {
            0
        } else {
            2 + longest
        }
    }
}
