/// https://leetcode.com/problems/maximum-length-of-repeated-subarray/
/// 
/// Time Complexity:    O(`n_nums` ^ 2)
/// Space Complexity:   O(`n_nums`)
/// 
/// Reference:
/// https://leetcode.com/problems/maximum-length-of-repeated-subarray/discuss/109068/JavaC%2B%2B-Clean-Code-8-lines
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let n_nums = nums1.len();
        
        let mut longest: u32 = 0;

        let mut cur_longest: u32 = 0;
        let mut idx_start: usize = 0;
        
        let mut dp: Vec<u32> = vec![0; n_nums + 1];
        
        for (idx1, num1) in nums1.iter().enumerate().rev(){
            for (idx2, num2) in nums2.iter().enumerate(){
                dp[idx2] = if num1 == num2 { 1 + dp[1 + idx2] } else { 0 };
                longest = std::cmp::max(longest, dp[idx2]);
                // to keep track of the subvector
                if longest > cur_longest{
                    idx_start = idx1;
                    cur_longest = longest;
                }
            }
        }
        
        // to extract the subvector
        (&nums1[idx_start..idx_start + longest as usize]).to_vec()
    }
}