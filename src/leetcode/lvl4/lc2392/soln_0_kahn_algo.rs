use std::collections::{HashMap, VecDeque};

/// @author: Leon
/// https://leetcode.com/problems/build-a-matrix-with-conditions/
/// Time Complexity:    O(V + E) + O(`k`) ~ O(`k` + `_len_rcs` + `_len_ccs`)
/// Space Complexity:   O(V + E) + O(`k`) ~ O(`k` + `_len_rcs` + `_len_ccs`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn build_matrix(
        k: i32,
        row_conditions: Vec<Vec<i32>>,
        col_conditions: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let _len_rcs: usize = row_conditions.len();
        let _len_ccs: usize = col_conditions.len();
        let k: usize = k as usize;
        let n: usize = k + 1;
        let (is_cyclic_r, map_r) = {
            let (graph, mut indegrees) = Self::build_graph(&row_conditions, n);
            Self::is_cyclic(&graph, &mut indegrees)
        };
        if is_cyclic_r {
            return Vec::new();
        }
        let (is_cyclic_c, map_c) = {
            let (graph, mut indegrees) = Self::build_graph(&col_conditions, n);
            Self::is_cyclic(&graph, &mut indegrees)
        };
        if is_cyclic_c {
            return Vec::new();
        }
        return Self::build_mat_simple(&map_r, &map_c, k);
    }
    /// to be improved
    /// this could pass for now,
    /// because of test cases being weak
    fn build_mat_simple(
        map_r: &HashMap<i32, usize>,
        map_c: &HashMap<i32, usize>,
        k: usize,
    ) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![vec![0; k]; k];
        for num in 1..=k as i32 {
            let r: usize = *map_r.get(&num).unwrap();
            let c: usize = *map_c.get(&num).unwrap();
            ans[r][c] = num;
        }
        return ans;
    }
    fn is_cyclic(graph: &Vec<Vec<usize>>, indegrees: &mut Vec<u16>) -> (bool, HashMap<i32, usize>) {
        let len_vs: usize = indegrees.len();
        let mut queue: VecDeque<usize> = {
            let mut queue = VecDeque::with_capacity(len_vs);
            for idx in 1..len_vs {
                if indegrees[idx] == 0 {
                    queue.push_back(idx);
                }
            }
            queue
        };
        let mut sorted: Vec<i32> = Vec::with_capacity(len_vs);
        while let Some(cur) = queue.pop_front() {
            sorted.push(cur as i32);
            for &nxt in &graph[cur] {
                indegrees[nxt] -= 1;
                if indegrees[nxt] == 0 {
                    queue.push_back(nxt);
                }
            }
        }
        if sorted.len() < len_vs - 1 {
            return (true, HashMap::new());
        }
        let num_to_idx: HashMap<i32, usize> = {
            let mut map: HashMap<i32, usize> = HashMap::with_capacity(len_vs);
            for (idx, num) in sorted.into_iter().enumerate() {
                map.insert(num, idx);
            }
            map
        };
        return (false, num_to_idx);
    }
    fn build_graph(edges: &Vec<Vec<i32>>, n: usize) -> (Vec<Vec<usize>>, Vec<u16>) {
        let mut graph: Vec<Vec<usize>> = vec![Vec::with_capacity(n); n];
        let mut indegrees: Vec<u16> = vec![0; n];
        for edge in edges {
            let from: usize = edge[0] as usize;
            let to: usize = edge[1] as usize;
            graph[from].push(to);
            indegrees[to] += 1;
        }
        return (graph, indegrees);
    }
}
