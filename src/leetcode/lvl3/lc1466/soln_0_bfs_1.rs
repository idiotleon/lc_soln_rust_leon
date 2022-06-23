use std::collections::{HashSet, VecDeque};

/// @author: Leon
/// https://leetcode.com/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero/
/// Time Complexity:    O(V + E) ~ O(`n` + `_len_cns`)
/// Space Complexity:   O(V + E) ~ O(`n` + `_len_cns`)
/// Reference:
/// https://leetcode.com/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero/discuss/661688/Java-Simple-BFS-from-Origin
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n: usize = n as usize;
        let _len_cns: usize = connections.len();
        let (graph, hashes) = Self::build_graph(n, &connections);
        let mut queue: VecDeque<usize> = {
            let mut queue: VecDeque<usize> = VecDeque::new();
            queue.push_back(0);
            queue
        };
        let mut visited: Vec<bool> = {
            let mut visited: Vec<bool> = vec![false; n as usize];
            visited[0] = true;
            visited
        };
        let mut cnt: i32 = 0;
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some(cur) = queue.pop_front() {
                    for &nxt in &graph[cur] {
                        if visited[nxt] {
                            continue;
                        }
                        visited[nxt] = true;
                        let hash = format!("{},{}", nxt, cur);
                        if !hashes.contains(&hash) {
                            cnt += 1;
                        }
                        queue.push_back(nxt);
                    }
                }
            }
        }
        cnt
    }
    fn build_graph(n: usize, edges: &Vec<Vec<i32>>) -> (Vec<Vec<usize>>, HashSet<String>) {
        let len_es: usize = edges.len();
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        let mut hashes: HashSet<String> = HashSet::with_capacity(len_es);
        for edge in edges {
            let u = edge[0];
            let v = edge[1];
            let hash = format!("{},{}", u, v);
            hashes.insert(hash);
            graph[u as usize].push(v as usize);
            graph[v as usize].push(u as usize);
        }
        (graph, hashes)
    }
}
