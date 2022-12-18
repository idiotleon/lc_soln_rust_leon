use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/daily-temperatures/
/// Time Complexity:    O(`len_t`)
/// Space Complexity:   O(`len_t`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn daily_temperatures(ts: Vec<i32>) -> Vec<i32> {
        let len_ts: usize = ts.len();
        let mut stk: VecDeque<usize> = VecDeque::with_capacity(len_ts);
        let mut ans: Vec<i32> = vec![0; len_ts];
        for (idx, &num) in ts.iter().enumerate() {
            while let Some(&idx_prev) = stk.back() {
                if ts[idx_prev] < num {
                    ans[idx_prev] = (idx - idx_prev) as i32;
                    stk.pop_back();
                } else {
                    break;
                }
            }
            stk.push_back(idx);
        }
        while let Some(idx) = stk.pop_back() {
            ans[idx] = 0;
        }
        return ans;
    }
}
