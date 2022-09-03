use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/maximum-number-of-robots-within-budget/
/// Time Complexity:    O(`len_rs`)
/// Space Complexity:   O(`len_rs`)
/// Reference:
/// https://leetcode.com/problems/maximum-number-of-robots-within-budget/discuss/2524838/JavaC%2B%2BPython-Sliding-Window-O(n)-Solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
        let len_rs: usize = charge_times.len();
        let mut sum: i64 = 0;
        let mut lo: usize = 0;
        let mut hi: usize = 0;
        let mut deque: VecDeque<usize> = VecDeque::with_capacity(len_rs);
        while hi < len_rs {
            sum += running_costs[hi] as i64;
            while let Some(&idx_last) = deque.back() {
                if charge_times[idx_last] <= charge_times[hi] {
                    deque.pop_back();
                } else {
                    break;
                }
            }
            deque.push_back(hi);
            if let Some(&idx_first) = deque.front() {
                if charge_times[idx_first] as i64 + (hi - lo + 1) as i64 * sum > budget {
                    if idx_first == lo {
                        deque.pop_front();
                    }
                    sum -= running_costs[lo] as i64;
                    lo += 1;
                }
            }
            hi += 1;
        }
        return (len_rs - lo) as i32;
    }
}
