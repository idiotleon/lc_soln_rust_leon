// @author: Leon
// https://leetcode.com/problems/number-of-connected-components-in-an-undirected-graph/
//
// Time Complexity:     O(V + E)
// Space Complexity:    O(V + E)
use std::collections::HashSet;
use std::collections::VecDeque;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let graph = Self::build_graph(n as usize, &edges);
        let mut visited: HashSet<i32> = HashSet::with_capacity(n as usize);
        let mut cnt = 0;

        for i in 0..n {
            if visited.contains(&i) {
                continue;
            }

            Self::bfs(i, &mut visited, &graph);
            cnt += 1;
        }

        cnt
    }

    fn bfs(start: i32, visited: &mut HashSet<i32>, graph: &Vec<Vec<i32>>) {
        let mut queue: VecDeque<i32> = VecDeque::new();
        queue.push_back(start);

        while !queue.is_empty() {
            let len_lvl = queue.len();

            for _ in 0..len_lvl {
                if let Some(top) = queue.pop_front() {
                    if !visited.insert(top) {
                        continue;
                    }

                    for &next in graph[top as usize].iter() {
                        queue.push_back(next);
                    }
                }
            }
        }
    }

    fn build_graph(n: usize, edges: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut graph: Vec<Vec<i32>> = vec![Vec::with_capacity(n); n];

        for edge in edges.iter() {
            let u = edge[0];
            let v = edge[1];

            graph[u as usize].push(v);
            graph[v as usize].push(u);
        }

        graph
    }
}