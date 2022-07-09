use std::collections::{HashMap, HashSet};

/// @author: Leon
/// https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/
/// Time Complexity:    O(V + E) ~ O(`len_sts` + )
/// Space Complexity:   O(V + E) ~ O(`len_sts` + )
/// Reference:
/// https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/discuss/197668/Count-the-Number-of-Islands-O(N)/269684
/// https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/discuss/197668/Count-the-Number-of-Islands-O(N)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let len_sts: usize = stones.len();
        let graph = Self::build_graph(&stones);
        let mut seen: HashSet<i32> = HashSet::with_capacity(len_sts);
        let mut cnt: i32 = 0;
        for stone in stones {
            for idx in 0..2 {
                let u = if idx == 0 { stone[0] } else { !stone[1] };
                if seen.insert(u) {
                    cnt += 1;
                    Self::dfs(u, &mut seen, &graph);
                }
            }
        }
        len_sts as i32 - cnt
    }
    fn dfs(cur: i32, seen: &mut HashSet<i32>, graph: &HashMap<i32, Vec<i32>>) {
        if let Some(nxts) = graph.get(&cur) {
            for &nxt in nxts {
                if seen.insert(nxt) {
                    Self::dfs(nxt, seen, graph);
                }
            }
        }
    }
    fn build_graph(stones: &Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
        let len_sts: usize = stones.len();
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::with_capacity(len_sts);
        for stone in stones {
            let u = stone[0];
            let v = !stone[1];
            graph.entry(u).or_default().push(v);
            graph.entry(v).or_default().push(u);
        }
        graph
    }
}
