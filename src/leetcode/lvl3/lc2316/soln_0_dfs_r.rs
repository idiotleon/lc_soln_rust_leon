use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/count-unreachable-pairs-of-nodes-in-an-undirected-graph/
/// Time Complexity:    O(V + E) ~ O(`n` + `_len_es`)
/// Space Complexity:   O(V + E) ~ O(`n` + `_len_es`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_pairs(n: i32, edges: Vec<Vec<i32>>) -> i64 {
        if edges.is_empty() {
            return n as i64 * (n as i64 - 1) / 2;
        }
        let n: usize = n as usize;
        let _len_es: usize = edges.len();
        let graph: Vec<Vec<usize>> = Self::build_graph(n, edges);
        let mut seen: HashSet<usize> = HashSet::new();
        let mut freqs: Vec<i64> = Vec::with_capacity(n);
        for idx in 0..n {
            if seen.insert(idx) {
                let cnt: i64 = Self::dfs(idx, &mut seen, &graph);
                freqs.push(cnt);
            }
        }
        Self::get_count(freqs)
    }
    fn get_count(freqs: Vec<i64>) -> i64 {
        let mut cnt: i64 = 0;
        let len_fs: usize = freqs.len();
        for lo in 0..len_fs - 1 {
            for hi in lo + 1..len_fs {
                cnt += freqs[lo] * freqs[hi];
            }
        }
        cnt
    }
    fn dfs(cur: usize, seen: &mut HashSet<usize>, graph: &Vec<Vec<usize>>) -> i64 {
        let mut cnt: i64 = 1;
        for &nxt in &graph[cur] {
            if seen.insert(nxt) {
                cnt += Self::dfs(nxt, seen, graph);
            }
        }
        cnt
    }
    fn build_graph(n: usize, edges: Vec<Vec<i32>>) -> Vec<Vec<usize>> {
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        for edge in edges {
            let u: usize = edge[0] as usize;
            let v: usize = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }
        graph
    }
}
