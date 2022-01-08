use std::collections::HashSet;
/// @author: Leon
/// https://leetcode.com/problems/find-if-path-exists-in-graph/
/// Time Complexity:    O(V + E) ~ O(`n` + `_len_es`)
/// Space Complexity:   O(V + E) ~ O(`n` + `_len_es`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, start: i32, end: i32) -> bool {
        let _len_es = edges.len();
        let graph: Vec<Vec<usize>> = {
            let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n as usize];
            for edge in edges {
                let u = edge[0] as usize;
                let v = edge[1] as usize;
                graph[u].push(v);
                graph[v].push(u);
            }
            graph
        };
        let mut seen: HashSet<usize> = HashSet::with_capacity(n as usize);
        seen.insert(start as usize);
        Self::dfs(start as usize, end as usize, &mut seen, &graph)
    }
    fn dfs(cur: usize, end: usize, seen: &mut HashSet<usize>, graph: &Vec<Vec<usize>>) -> bool {
        if cur == end {
            return true;
        }
        for &next in &graph[cur] {
            if seen.insert(next) {
                if Self::dfs(next, end, seen, graph) {
                    return true;
                }
            }
        }
        false
    }
}
