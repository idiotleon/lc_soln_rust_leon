/// @author: Leon
/// https://leetcode.com/problems/crawler-log-folder/
/// Time Complexity:    O(`_len_ls` * avg_len_log)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let _len_ls: usize = logs.len();
        let mut ans: i32 = 0;
        const PREV: &str = &"../";
        const CUR: &str = &"./";
        for log in logs {
            match &log[..] {
                PREV => {
                    ans -= 1;
                    ans = std::cmp::max(ans, 0);
                }
                CUR => {
                    // to explicitly do nothing
                }
                _ => {
                    ans += 1;
                }
            }
        }
        return ans;
    }
}
