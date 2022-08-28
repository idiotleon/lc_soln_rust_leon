/// @author: Leon
/// https://leetcode.com/problems/longest-subsequence-with-limited-sum/
/// Time Complexity:    O(`len_ns` * lg(`len_ns`)) + O(`len_ns` * `len_qs`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let len_ns: usize = nums.len();
        let len_qs: usize = queries.len();
        let nums: Vec<i32> = {
            let mut nums = nums;
            nums.sort();
            nums
        };
        let mut ans: Vec<i32> = vec![0; len_qs];
        for (idx_q, query) in queries.into_iter().enumerate() {
            let mut sum = 0;
            for idx_n in 0..len_ns {
                sum += nums[idx_n];
                if sum > query {
                    ans[idx_q] = idx_n as i32;
                    break;
                } else {
                    ans[idx_q] = idx_n as i32 + 1;
                }
            }
        }
        return ans;
    }
}
