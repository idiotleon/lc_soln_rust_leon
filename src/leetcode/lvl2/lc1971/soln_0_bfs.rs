use std::collections::{HashSet, VecDeque};
/// @author: Leon
/// https://leetcode.com/problems/find-if-path-exists-in-graph/
/// Time Complexity:    O(V + E) ~ O(`n` + `n_edges`)
/// Space Complexity:   O(V + E) ~ O(`n` + `n_edges`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, start: i32, end: i32) -> bool {
        let graph: Vec<HashSet<usize>> = {
            let mut graph: Vec<HashSet<usize>> = vec![HashSet::new(); n as usize];
            for edge in edges.into_iter() {
                let u = edge[0] as usize;
                let v = edge[1] as usize;
                graph[u].insert(v);
                graph[v].insert(u);
            }
            graph
        };
        let mut queue: VecDeque<usize> = VecDeque::with_capacity(n as usize);
        queue.push_back(start as usize);
        let mut seen: HashSet<usize> = HashSet::with_capacity(n as usize);
        seen.insert(start as usize);
        while !queue.is_empty() {
            let size = queue.len();
            for _ in 0..size {
                if let Some(cur) = queue.pop_front() {
                    if cur == end as usize {
                        return true;
                    }
                    for &next in graph[cur].iter() {
                        if seen.insert(next) {
                            queue.push_back(next);
                        }
                    }
                }
            }
        }
        false
    }
}
