use std::collections::{HashSet, VecDeque};

/// @author: Leon
/// https://leetcode.com/problems/graph-valid-tree/
/// Time Complexity:    O(V + E) ~ O(`n` + `len_es`)
/// Space Complexity:   O(V + E) ~ O(`n` + `len_es`)
/// This approach relies on the basic that:
/// it has to be exactly (`n` - 1) edges, so that
/// 1. there is no cycle,
/// 2. all the vertices are connected.
/// Reference:
/// https://leetcode.com/problems/graph-valid-tree/editorial/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let n: usize = n as usize;
        let len_es: usize = edges.len();
        if len_es != n - 1 {
            return false;
        }
        let graph = Self::build_graph(n, edges);
        return Self::bfs(n, &graph);
    }
    fn bfs(n: usize, graph: &Vec<Vec<usize>>) -> bool {
        let mut queue: VecDeque<usize> = {
            let mut queue: VecDeque<usize> = VecDeque::with_capacity(n);
            queue.push_back(0);
            queue
        };
        let mut seen: HashSet<usize> = {
            let mut seen: HashSet<usize> = HashSet::with_capacity(n);
            seen.insert(0);
            seen
        };
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some(cur) = queue.pop_front() {
                    for &nxt in &graph[cur] {
                        if seen.insert(nxt) {
                            queue.push_back(nxt);
                        }
                    }
                }
            }
        }
        return seen.len() == n;
    }
    fn build_graph(n: usize, edges: Vec<Vec<i32>>) -> Vec<Vec<usize>> {
        let mut graph: Vec<Vec<usize>> = vec![Vec::with_capacity(n); n];
        for edge in edges {
            let u: usize = edge[0] as usize;
            let v: usize = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }
        return graph;
    }
}
