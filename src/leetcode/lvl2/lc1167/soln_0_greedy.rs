/// https://leetcode.com/problems/minimum-cost-to-connect-sticks/
///
/// Time Complexity:    O(`len_s` * lg(`len_s`))
/// Space Complexity:   O(`len_s`)
///
/// Reference:
/// https://leetcode.com/problems/minimum-cost-to-connect-sticks/discuss/365865/Python-Greedy-Solution
use std::collections::BinaryHeap;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn connect_sticks(sticks: Vec<i32>) -> i32 {
        // not used
        // let len_s = sticks.len();

        let mut heap = BinaryHeap::<i32>::new();

        for stick in sticks {
            heap.push(-stick);
        }
        let mut min_cost: i32 = 0;

        while heap.len() > 1 {
            let conn_cost = -heap.pop().unwrap() - heap.pop().unwrap();
            min_cost += conn_cost;
            heap.push(-conn_cost);
        }
        min_cost
    }
}
