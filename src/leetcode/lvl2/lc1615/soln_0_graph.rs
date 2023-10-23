use std::collections::{HashMap, HashSet};

/// @author: Leon
/// https://leetcode.com/problems/maximal-network-rank/
/// Time Complexity:    O(E + V ^ 2)
/// Space Complexity:   O(E + V)
/// Reference:
/// https://leetcode.com/problems/maximal-network-rank/editorial/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n: usize = n as usize;
        let graph: HashMap<usize, HashSet<usize>> = Self::build_graph(n, roads);
        let mut ans: i32 = 0;
        for node1 in 0..n - 1 {
            for node2 in node1 + 1..n {
                let rank = {
                    let rank1: i32 = match graph.get(&node1) {
                        Some(nodes) => nodes.len() as i32,
                        None => 0,
                    };
                    let rank2: i32 = match graph.get(&node2) {
                        Some(nodes) => nodes.len() as i32,
                        None => 0,
                    };
                    let mut rank: i32 = rank1 + rank2;
                    if let Some(nodes) = graph.get(&node1) {
                        if nodes.contains(&node2) {
                            rank -= 1;
                        }
                    }
                    rank as i32
                };
                ans = std::cmp::max(ans, rank);
            }
        }
        return ans;
    }
    fn build_graph(n: usize, roads: Vec<Vec<i32>>) -> HashMap<usize, HashSet<usize>> {
        let mut graph: HashMap<usize, HashSet<usize>> = HashMap::with_capacity(n);
        for road in roads {
            let u: usize = road[0] as usize;
            let v: usize = road[1] as usize;
            graph
                .entry(u)
                .or_insert(HashSet::with_capacity(n))
                .insert(v);
            graph
                .entry(v)
                .or_insert(HashSet::with_capacity(n))
                .insert(u);
        }
        return graph;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let n: i32 = 4;
        let roads: Vec<Vec<i32>> = vec![vec![0, 1], vec![0, 3], vec![1, 2], vec![1, 3]];
        let expected: i32 = 4;
        let actual = Solution::maximal_network_rank(n, roads);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let n: i32 = 5;
        let roads: Vec<Vec<i32>> = vec![
            vec![0, 1],
            vec![0, 3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
            vec![2, 4],
        ];
        let expected: i32 = 5;
        let actual = Solution::maximal_network_rank(n, roads);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_3() {
        let n: i32 = 8;
        let roads: Vec<Vec<i32>> = vec![
            vec![0, 1],
            vec![1, 2],
            vec![2, 3],
            vec![2, 4],
            vec![5, 6],
            vec![5, 7],
        ];
        let expected: i32 = 5;
        let actual = Solution::maximal_network_rank(n, roads);
        assert_eq!(expected, actual);
    }
}
