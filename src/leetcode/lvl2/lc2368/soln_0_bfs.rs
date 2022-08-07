use std::collections::{HashSet, VecDeque};

/// @author: Leon
/// https://leetcode.com/problems/reachable-nodes-with-restrictions/
/// Time Complexity:    O(V + E) ~ O(`n` + `_len_es`)
/// Space Complexity:   O(V + E) ~ O(`n` + `_len_es`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        let _len_es: usize = edges.len();
        let n: usize = n as usize;
        let graph: Vec<Vec<usize>> = Self::build_graph(n as usize, &edges);
        let mut seen: HashSet<usize> = {
            let mut seen: HashSet<usize> = restricted.into_iter().map(|e| e as usize).collect();
            seen.insert(0);
            seen
        };
        let mut res: Vec<usize> = {
            let mut res: Vec<usize> = Vec::with_capacity(n);
            res.push(0);
            res
        };
        let mut queue: VecDeque<usize> = {
            let mut queue = VecDeque::with_capacity(n);
            queue.push_back(0);
            queue
        };
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some(cur) = queue.pop_front() {
                    for &nxt in &graph[cur] {
                        if seen.insert(nxt) {
                            res.push(nxt);
                            queue.push_back(nxt);
                        }
                    }
                }
            }
        }
        res.len() as i32
    }
    fn build_graph(n: usize, edges: &Vec<Vec<i32>>) -> Vec<Vec<usize>> {
        let mut graph: Vec<Vec<usize>> = vec![Vec::with_capacity(n); n];
        for edge in edges {
            let u: usize = edge[0] as usize;
            let v: usize = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }
        graph
    }
}
