use std::collections::{HashMap, HashSet};

/// @author: Leon
/// https://leetcode.com/problems/restore-the-array-from-adjacent-pairs/
/// Time Complexity:    O(V + E) ~ O(`len_aps` * 2 + `len_aps`) ~ O(`len_aps`)
/// Space Complexity:   O(V + E) ~ O(`len_aps` * 2 + `len_aps`) ~ O(`len_aps`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn restore_array(adjacent_pairs: Vec<Vec<i32>>) -> Vec<i32> {
        let len_aps: usize = adjacent_pairs.len();
        let (graph, counts): (HashMap<i32, Vec<i32>>, HashMap<i32, i32>) =
            Self::build_graph(&adjacent_pairs);
        let mut seen: HashSet<i32> = HashSet::with_capacity(len_aps * 2);
        let mut ans: Vec<i32> = Vec::with_capacity(len_aps);
        let start: i32 = {
            let mut res: i32 = 0;
            for (idx, count) in counts.into_iter() {
                if count == 1 {
                    res = idx;
                    break;
                }
            }
            res
        };
        Self::dfs(start, &mut seen, &graph, &mut ans);
        return ans;
    }
    fn dfs(cur: i32, seen: &mut HashSet<i32>, graph: &HashMap<i32, Vec<i32>>, res: &mut Vec<i32>) {
        if !seen.insert(cur) {
            return;
        }
        res.push(cur);
        if let Some(nxts) = graph.get(&cur) {
            for &nxt in nxts {
                Self::dfs(nxt, seen, graph, res);
            }
        }
    }
    fn build_graph(adj_pairs: &Vec<Vec<i32>>) -> (HashMap<i32, Vec<i32>>, HashMap<i32, i32>) {
        let len_ps: usize = adj_pairs.len();
        let mut map: HashMap<i32, Vec<i32>> = HashMap::with_capacity(len_ps * 2);
        let mut counts: HashMap<i32, i32> = HashMap::with_capacity(len_ps * 2);
        for adj in adj_pairs {
            let u: i32 = adj[0];
            let v: i32 = adj[1];
            map.entry(u).or_default().push(v);
            map.entry(v).or_default().push(u);
            *counts.entry(u).or_default() += 1;
            *counts.entry(v).or_default() += 1;
        }
        return (map, counts);
    }
}
