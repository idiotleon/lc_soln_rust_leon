/// https://leetcode.com/problems/minimum-height-trees/
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(`n`)
/// Referenece:
/// https://leetcode.com/problems/minimum-height-trees/discuss/76129/Share-my-BFS-JAVA-code-using-degree-with-explanation-which-beats-more-than-95/293110
#[allow(dead_code)]
struct Solution;

use std::collections::VecDeque;

#[allow(dead_code)]
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i16>>) -> Vec<i16> {
        let n: usize = n as usize;
        if n == 1 {
            return vec![0];
        }
        let mut degrees = vec![0; n];
        let graph = Self::build_graph(&mut degrees, n as usize, &edges);
        let mut queue = VecDeque::<i16>::new();
        for (idx_node, &degree) in degrees.iter().enumerate() {
            if degree == 1 {
                queue.push_back(idx_node as i16);
            }
        }
        let ans: Vec<i16> = {
            let mut last: Vec<i16> = vec![];
            while !queue.is_empty() {
                let len_q = queue.len();
                let mut leaves: Vec<i16> = vec![];
                for _ in 0..len_q {
                    if let Some(cur) = queue.pop_front() {
                        leaves.push(cur);
                        for &next in &graph[cur as usize] {
                            degrees[next as usize] -= 1;
                            if degrees[next as usize] == 1 {
                                queue.push_back(next);
                            }
                        }
                    }
                }
                last = leaves;
            }
            last
        };
        ans
    }
    fn build_graph(degrees: &mut Vec<u16>, n: usize, edges: &Vec<Vec<i16>>) -> Vec<Vec<i16>> {
        let graph: Vec<Vec<i16>> = {
            let mut graph = vec![vec![]; n];
            for edge in edges {
                let u = edge[0];
                let v = edge[1];
                graph[u as usize].push(v as i16);
                degrees[u as usize] += 1;
                graph[v as usize].push(u as i16);
                degrees[v as usize] += 1;
            }
            graph
        };
        graph
    }
}
