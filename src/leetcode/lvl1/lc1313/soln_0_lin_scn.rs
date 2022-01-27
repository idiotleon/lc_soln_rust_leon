/// @author: Leon
/// https://leetcode.com/problems/decompress-run-length-encoded-list/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1) / O(`len_n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let len_n: usize = nums.len();
        let mut ans: Vec<i32> = Vec::new();
        for idx in (0..len_n).step_by(2) {
            for _ in 0..nums[idx] {
                ans.push(nums[idx + 1]);
            }
        }
        ans
    }
}
