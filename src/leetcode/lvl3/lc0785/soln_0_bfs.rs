use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/is-graph-bipartite/
/// Time Complexity:    O(V + E)
/// Space Complexity:   O(V + E)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let n: usize = graph.len();
        let mut visited: Vec<i8> = vec![0; n];
        let mut flag: i8 = 1;
        for cur in 0..n {
            if visited[cur] != 0 {
                continue;
            }
            if !Self::bfs(cur, &mut visited, &mut flag, &graph) {
                return false;
            }
        }
        true
    }

    fn bfs(node: usize, visited: &mut Vec<i8>, flag: &mut i8, graph: &Vec<Vec<i32>>) -> bool {
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(node);
        visited[node] = *flag;
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some(cur) = queue.pop_front() {
                    if visited[cur] != 0 && visited[cur] != *flag {
                        return false;
                    }
                    visited[cur] = *flag;
                    for &nxt in &graph[cur] {
                        if visited[nxt as usize] != 0 {
                            continue;
                        }
                        queue.push_back(nxt as usize);
                    }
                }
            }
            *flag *= -1;
        }
        true
    }
}
