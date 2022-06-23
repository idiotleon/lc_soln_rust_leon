/// @author: Leon
/// https://leetcode.com/problems/maximum-length-of-repeated-subarray/
/// Time Complexity:    O(`n_nums` ^ 2)
/// Space Complexity:   O(`n_nums`)
/// Reference:
/// https://leetcode.com/problems/maximum-length-of-repeated-subarray/discuss/109068/JavaC%2B%2B-Clean-Code-8-lines
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n_nums = nums1.len();
        let mut longest: u16 = 0;
        let mut dp: Vec<u16> = vec![0; n_nums + 1];
        for (_idx1, num1) in nums1.iter().enumerate().rev() {
            for (idx2, num2) in nums2.iter().enumerate() {
                dp[idx2] = if num1 == num2 { 1 + dp[1 + idx2] } else { 0 };
                longest = std::cmp::max(longest, dp[idx2]);
            }
        }
        longest as i32
    }
}
