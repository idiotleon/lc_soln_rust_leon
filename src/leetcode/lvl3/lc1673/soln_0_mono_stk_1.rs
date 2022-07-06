/// @author: Leon
/// https://leetcode.com/problems/find-the-most-competitive-subsequence/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`k`)
/// Reference:
/// https://leetcode.com/problems/find-the-most-competitive-subsequence/discuss/1028519/C%2B%2B-No-Stack-Fastest-Solution-to-Date-(100ms)-Explained-100-Time-55-Space
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn most_competitive(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let len_ns: usize = nums.len();
        let k: usize = k as usize;
        let mut ans: Vec<i32> = vec![0; k];
        let mut idx_k: usize = 0;
        for (idx, num) in nums.into_iter().enumerate() {
            while idx_k > 0 && k - idx_k < len_ns - idx && ans[idx_k - 1] > num {
                idx_k -= 1;
            }
            if idx_k < k {
                ans[idx_k] = num;
                idx_k += 1;
            }
        }
        ans
    }
}
