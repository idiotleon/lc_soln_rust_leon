/// @author: Leon
/// https://leetcode.com/problems/maximum-length-of-repeated-subarray/
/// Time Complexity:    O(`len_n` ^ 2)
/// Space Complexity:   O(`len_n`)
/// Reference:
/// https://leetcode.com/problems/maximum-length-of-repeated-subarray/discuss/109068/JavaC%2B%2B-Clean-Code-8-lines
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let len_n = nums1.len();
        let mut longest: u16 = 0;
        let mut idx1_start: usize = 0;
        let mut dp: Vec<u16> = vec![0; len_n + 1];
        for (idx1, num1) in nums1.iter().enumerate().rev() {
            for (idx2, num2) in nums2.iter().enumerate() {
                dp[idx2] = if num1 == num2 { 1 + dp[1 + idx2] } else { 0 };
                // to keep track of the subvector
                if dp[idx2] > longest {
                    longest = dp[idx2];
                    idx1_start = idx1;
                }
            }
        }
        // to extract the subvector
        (&nums1[idx1_start..idx1_start + longest as usize]).to_vec()
    }
}
