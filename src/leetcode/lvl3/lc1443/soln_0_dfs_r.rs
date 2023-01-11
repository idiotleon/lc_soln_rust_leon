use std::collections::{HashMap, HashSet};

/// @author: Leon
/// https://leetcode.com/problems/minimum-time-to-collect-all-apples-in-a-tree/
/// Time Complexity:    O(`n` + `_len_es`)
/// Space Complexity:   O(`n` + `_len_es`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    const START: usize = 0;
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let _len_es: usize = edges.len();
        let graph: HashMap<usize, Vec<usize>> = Self::build_graph(n as usize, edges);
        let mut visited: HashSet<usize> = HashSet::with_capacity(n as usize);
        return Self::dfs(Self::START, &mut visited, &has_apple, &graph);
    }
    fn dfs(
        cur: usize,
        visited: &mut HashSet<usize>,
        has_apple: &Vec<bool>,
        graph: &HashMap<usize, Vec<usize>>,
    ) -> i32 {
        if visited.contains(&cur) {
            return 0;
        }
        visited.insert(cur);
        let mut cnt: i32 = 0;
        if let Some(nxts) = graph.get(&cur) {
            for &nxt in nxts {
                cnt += Self::dfs(nxt, visited, has_apple, graph);
            }
        }
        if (cnt > 0 || has_apple[cur]) && cur != Self::START {
            cnt += 2;
        }
        visited.remove(&cur);
        return cnt;
    }
    fn build_graph(n: usize, edges: Vec<Vec<i32>>) -> HashMap<usize, Vec<usize>> {
        let mut graph: HashMap<usize, Vec<usize>> = HashMap::with_capacity(n as usize);
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph.entry(u).or_default().push(v);
            graph.entry(v).or_default().push(u);
        }
        return graph;
    }
}
