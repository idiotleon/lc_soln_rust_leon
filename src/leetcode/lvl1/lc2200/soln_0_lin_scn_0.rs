/// @author: Leon
/// https://leetcode.com/problems/find-all-k-distant-indices-in-an-array/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(`1`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let len_n: usize = nums.len();
        const RANGE: i32 = 1000 + 1;
        let mut ans: Vec<i32> = Vec::new();
        for (idx, num) in nums.into_iter().enumerate() {
            if num == key {
                // the exclude the previous end index
                let prev_end: i32 = *ans.last().unwrap_or(&-1) + 1;
                let start: i32 = std::cmp::max(prev_end, idx as i32 - k);
                // to exclude the last index
                let end: i32 = std::cmp::min(len_n as i32 - 1, idx as i32 + k);
                for num in start..=end {
                    ans.push(num);
                }
            }
        }
        ans
    }
}
