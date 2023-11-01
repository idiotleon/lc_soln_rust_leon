use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/minimum-height-trees/
/// Time Complexity:    O(V + E) ~ O(`n` + `_len_es`) ~ O(`n`)
/// Space Complexity:   O(V + E) ~ O(`n` + `_len_es`) ~ O(`n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        if n == 1 {
            return vec![0];
        }
        let n: usize = n as usize;
        let (graph, mut counts) = Self::build_graph(n, edges);
        let mut queue: VecDeque<usize> = {
            let mut queue: VecDeque<usize> = VecDeque::with_capacity(n);
            for (idx, &count) in counts.iter().enumerate() {
                if count == 1 {
                    queue.push_back(idx);
                }
            }
            queue
        };
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            let mut res: Vec<i32> = Vec::new();
            for _ in 0..len_q {
                if let Some(cur) = queue.pop_front() {
                    res.push(cur as i32);
                    for &nxt in &graph[cur] {
                        counts[nxt] -= 1;
                        if counts[nxt] == 1 {
                            queue.push_back(nxt);
                        }
                    }
                }
            }
            ans = res;
        }
        return ans;
    }
    fn build_graph(n: usize, edges: Vec<Vec<i32>>) -> (Vec<Vec<usize>>, Vec<u16>) {
        let mut graph: Vec<Vec<usize>> = vec![Vec::with_capacity(n); n];
        let mut counts: Vec<u16> = vec![0; n];
        for edge in edges {
            let u: usize = edge[0] as usize;
            let v: usize = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
            counts[u] += 1;
            counts[v] += 1;
        }
        return (graph, counts);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let n: i32 = 4;
        let edges: Vec<Vec<i32>> = vec![vec![1, 0], vec![1, 2], vec![1, 3]];
        let expected: Vec<i32> = vec![1];
        let actual: Vec<i32> = Solution::find_min_height_trees(n, edges);
        assert_eq!(expected, actual);
    }
}
