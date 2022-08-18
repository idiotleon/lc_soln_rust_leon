use std::collections::{HashSet, VecDeque};

/// @author: Leon
/// https://leetcode.com/problems/all-ancestors-of-a-node-in-a-directed-acyclic-graph/
/// Time Complexity:    O(V + E) ~ O(`n` + `_len_es`)
/// Space Complexity:   O(V + E) ~ O(`n` + `_len_es`)
/// Reference:
/// https://leetcode.com/problems/all-ancestors-of-a-node-in-a-directed-acyclic-graph/discuss/1821991/JavaPython-3-2-codes%3ATopological-sort-and-DFS-w-brief-explanation-and-comments.
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_ancestors(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let _len_es: usize = edges.len();
        let n: usize = n as usize;
        let (graph, mut indegrees) = Self::build_graph(n, &edges);
        let mut ans: Vec<HashSet<i32>> = vec![HashSet::new(); n];
        let mut queue: VecDeque<usize> = {
            let mut queue: VecDeque<usize> = VecDeque::with_capacity(n);
            for (idx, &indegree) in indegrees.iter().enumerate() {
                if indegree == 0 {
                    queue.push_back(idx);
                }
            }
            queue
        };
        while let Some(cur) = queue.pop_front() {
            for &nxt in &graph[cur] {
                ans[nxt].insert(cur as i32);
                let ancestors = ans[cur].clone();
                ans[nxt].extend(&ancestors);
                indegrees[nxt] -= 1;
                if indegrees[nxt] == 0 {
                    queue.push_back(nxt);
                }
            }
        }
        return ans
            .into_iter()
            .map(|s| {
                let mut v: Vec<i32> = s.into_iter().collect();
                v.sort();
                v
            })
            .collect();
    }
    fn build_graph(n: usize, edges: &Vec<Vec<i32>>) -> (Vec<Vec<usize>>, Vec<u16>) {
        let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
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
