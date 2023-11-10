use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/exclusive-time-of-functions/
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(`n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        const START: &str = &"start";
        let n: usize = n as usize;
        let mut ans: Vec<i32> = vec![0; n];
        let mut stk: VecDeque<usize> = VecDeque::with_capacity(n);
        let mut ts_prev: i32 = 0;
        for log in logs {
            let res: Vec<&str> = log.split(':').collect();
            let id = res[0].parse::<usize>().unwrap();
            let state = res[1];
            let ts_cur = res[2].parse::<i32>().unwrap();
            if let Some(&top) = stk.back() {
                ans[top] += ts_cur - ts_prev;
            }
            ts_prev = ts_cur;
            if state == START {
                stk.push_back(id);
            } else {
                if let Some(top) = stk.pop_back() {
                    ans[top] += 1;
                }
                ts_prev += 1;
            }
        }
        return ans;
    }
}
