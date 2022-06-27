use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/find-eventual-safe-states/
/// Time Complexity:    O(V + E) ~ O(`len_vts` + len_es)
/// Space Complexity:   O(V + E) ~ O(`len_vts` + len_es)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let len_vts: usize = graph.len();
        let (adj, mut indegrees) = Self::build_graph(graph);
        let mut queue: VecDeque<usize> = {
            let mut queue: VecDeque<usize> = VecDeque::with_capacity(len_vts);
            for (idx, &indegree) in indegrees.iter().enumerate() {
                if indegree == 0 {
                    queue.push_back(idx);
                }
            }
            queue
        };
        let mut ans: Vec<i32> = Vec::with_capacity(len_vts);
        while !queue.is_empty() {
            if let Some(cur) = queue.pop_front() {
                ans.push(cur as i32);
                for &nxt in &adj[cur] {
                    indegrees[nxt] -= 1;
                    if indegrees[nxt] == 0 {
                        queue.push_back(nxt);
                    }
                }
            }
        }
        ans.sort();
        ans
    }
    fn build_graph(graph: Vec<Vec<i32>>) -> (Vec<Vec<usize>>, Vec<u16>) {
        let len_vts: usize = graph.len();
        let mut adj: Vec<Vec<usize>> = vec![Vec::with_capacity(len_vts); len_vts];
        let mut indegrees: Vec<u16> = vec![0; len_vts];
        for (idx, children) in graph.into_iter().enumerate() {
            for child in children {
                adj[child as usize].push(idx);
                indegrees[idx] += 1;
            }
        }
        (adj, indegrees)
    }
}
