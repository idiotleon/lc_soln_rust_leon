use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/graph-valid-tree/
/// Time Complexity:    O(V + E) ~ O(`n` + `_len_es`)
/// Space Complexity:   O(V + E) ~ O(`n` + `_len_es`)
/// Reference:
/// https://leetcode.com/problems/graph-valid-tree/editorial/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let _len_es: usize = edges.len();
        const RANGE: usize = 2000;
        // the impossible
        const IMPS: usize = RANGE + 7;
        let n: usize = n as usize;
        let graph = Self::build_graph(n, edges);
        let mut seen: HashSet<usize> = HashSet::with_capacity(n);
        return !Self::is_cyclic(0, IMPS, &mut seen, &graph) && seen.len() == n;
    }
    fn is_cyclic(
        cur: usize,
        prev: usize,
        seen: &mut HashSet<usize>,
        graph: &Vec<Vec<usize>>,
    ) -> bool {
        if !seen.insert(cur) {
            return true;
        }
        for &nxt in &graph[cur] {
            if nxt != prev {
                let is_cyclic = Self::is_cyclic(nxt, cur, seen, graph);
                if is_cyclic {
                    return true;
                }
            }
        }
        return false;
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
