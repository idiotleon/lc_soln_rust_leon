use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/possible-bipartition/
/// Time Complexity:    O(V + E) ~ O(`n` + `_len_dks`)
/// Space Complexity:   O(V + E) ~ O(`n` + `_len_dks`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    const NOT_VISITED: i8 = 0;
    const FLAG: i8 = 1;
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        let _len_dks: usize = dislikes.len();
        let graph = Self::build_graph(n as usize, &dislikes);
        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut visited: Vec<i8> = vec![Self::NOT_VISITED; n as usize + 1];
        for node in 1..=n as usize {
            if visited[node] != Self::NOT_VISITED {
                continue;
            }
            if !Self::bfs(node, &mut queue, &mut visited, &graph) {
                return false;
            }
        }
        true
    }
    fn bfs(
        node: usize,
        queue: &mut VecDeque<usize>,
        visited: &mut Vec<i8>,
        graph: &Vec<Vec<usize>>,
    ) -> bool {
        queue.push_back(node);
        visited[node] = Self::FLAG;
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some(cur) = queue.pop_front() {
                    for &nxt in &graph[cur] {
                        if visited[nxt] != Self::NOT_VISITED {
                            if visited[cur] == -visited[nxt] {
                                continue;
                            } else {
                                return false;
                            }
                        }
                        queue.push_back(nxt);
                        visited[nxt] = -visited[cur];
                    }
                }
            }
        }
        true
    }
    fn build_graph(n: usize, edges: &Vec<Vec<i32>>) -> Vec<Vec<usize>> {
        let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
        for edge in edges {
            let u: usize = edge[0] as usize;
            let v: usize = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }
        graph
    }
}
