use std::collections::{HashMap, VecDeque};

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
        const RANGE: usize = 2000;
        // the impossible
        const IMPS: usize = RANGE + 7;
        let n: usize = n as usize;
        let graph = Self::build_graph(n, edges);
        let mut child_to_parent: HashMap<usize, usize> = {
            let mut map = HashMap::with_capacity(n);
            map.insert(0, IMPS);
            map
        };
        return !Self::is_cyclic(&mut child_to_parent, n, &graph) && child_to_parent.len() == n;
    }
    fn is_cyclic(
        child_to_parent: &mut HashMap<usize, usize>,
        n: usize,
        graph: &Vec<Vec<usize>>,
    ) -> bool {
        let mut queue: VecDeque<usize> = {
            let mut queue: VecDeque<usize> = VecDeque::with_capacity(n);
            queue.push_back(0);
            queue
        };
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some(cur) = queue.pop_front() {
                    for &nxt in &graph[cur] {
                        if let Some(&parent) = child_to_parent.get(&cur) {
                            if parent == nxt {
                                continue;
                            }
                            if child_to_parent.contains_key(&nxt) {
                                return true;
                            }
                            queue.push_back(nxt);
                            child_to_parent.insert(nxt, cur);
                        }
                    }
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
