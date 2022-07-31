use std::collections::{HashSet, VecDeque};

/// @author: Leon
/// https://leetcode.com/problems/find-closest-node-to-given-two-nodes/
/// Time Complexity:    O(V + E) ~ O(`len_vs`)
/// Space Complexity:   O(V + E) ~ O(`len_vs`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let len_vs: usize = edges.len();
        let mut queue1: VecDeque<i32> = VecDeque::with_capacity(len_vs);
        queue1.push_back(node1);
        let mut seen1: HashSet<i32> = HashSet::with_capacity(len_vs);
        seen1.insert(node1);
        let mut queue2: VecDeque<i32> = VecDeque::with_capacity(len_vs);
        queue2.push_back(node2);
        let mut seen2: HashSet<i32> = HashSet::with_capacity(len_vs);
        seen2.insert(node2);
        while !queue1.is_empty() || !queue2.is_empty() {
            let mut res: Vec<i32> = Vec::new();
            let len_q1: usize = queue1.len();
            for _ in 0..len_q1 {
                if let Some(cur) = queue1.pop_front() {
                    if seen2.contains(&cur) {
                        res.push(cur);
                    }
                    let nxt = edges[cur as usize];
                    if nxt != -1 && seen1.insert(nxt) {
                        queue1.push_back(nxt);
                    }
                }
            }
            let len_q2: usize = queue2.len();
            for _ in 0..len_q2 {
                if let Some(cur) = queue2.pop_front() {
                    if seen1.contains(&cur) {
                        res.push(cur);
                    }
                    let nxt = edges[cur as usize];
                    if nxt != -1 && seen2.insert(nxt) {
                        queue2.push_back(nxt);
                    }
                }
            }
            if !res.is_empty() {
                return res.into_iter().min().unwrap();
            }
        }
        -1
    }
}
