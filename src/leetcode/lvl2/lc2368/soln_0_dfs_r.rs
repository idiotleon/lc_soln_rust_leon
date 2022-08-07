use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/reachable-nodes-with-restrictions/
/// Time Complexity:    O(V + E) ~ O(`n` + `_len_es`)
/// Space Complexity:   O(V + E) ~ O(`n` + `_len_es`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reachable_nodes(n: i32, edges: Vec<Vec<i32>>, restricted: Vec<i32>) -> i32 {
        let _len_es: usize = edges.len();
        let graph: Vec<Vec<usize>> = Self::build_graph(n as usize, &edges);
        let mut seen: HashSet<usize> = {
            let mut seen: HashSet<usize> = restricted.into_iter().map(|e| e as usize).collect();
            seen.insert(0);
            seen
        };
        let mut res: Vec<usize> = vec![0];
        Self::dfs(0, &mut seen, &graph, &mut res);
        res.len() as i32
    }
    fn dfs(cur: usize, seen: &mut HashSet<usize>, graph: &Vec<Vec<usize>>, res: &mut Vec<usize>) {
        for &nxt in &graph[cur] {
            if seen.insert(nxt) {
                res.push(nxt);
                Self::dfs(nxt, seen, graph, res);
            }
        }
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
