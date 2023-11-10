use std::collections::{HashMap, HashSet, VecDeque};

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
        return Self::bfs(start, &mut seen, &graph);
    }
    fn bfs(start: i32, seen: &mut HashSet<i32>, graph: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
        let len_vtx: usize = graph.len();
        let mut ans: Vec<i32> = Vec::with_capacity(len_vtx);
        let mut queue: VecDeque<i32> = {
            let mut queue: VecDeque<i32> = VecDeque::with_capacity(len_vtx);
            queue.push_back(start);
            queue
        };
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some(cur) = queue.pop_front() {
                    if seen.insert(cur) {
                        ans.push(cur);
                        if let Some(nxts) = graph.get(&cur) {
                            for &nxt in nxts {
                                queue.push_back(nxt);
                            }
                        };
                    }
                }
            }
        }
        return ans;
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
