use std::collections::{HashMap, VecDeque};
/// @author: Leon
/// https://leetcode.com/problems/parallel-courses/
/// Time Complexity:    O(V + E) ~ O(`n` + `_len_rlts`)
/// Space Complexity:   O(V + E) ~ O(`n` + `_len_rlts`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_semesters(n: i32, relations: Vec<Vec<i32>>) -> i32 {
        let _len_rlts: usize = relations.len();
        let (graph, mut indegrees) = Self::build_graph(&relations, n as usize);
        let mut queue: VecDeque<usize> = VecDeque::new();
        for (vertex, &degree) in indegrees.iter().enumerate() {
            if vertex == 0 {
                continue;
            }
            if degree == 0 {
                queue.push_back(vertex);
            }
        }

        let mut semesters: u16 = 0;
        let mut cnt: u16 = 0;
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some(cur) = queue.pop_front() {
                    if graph.contains_key(&cur) {
                        for &nxt in &graph[&cur] {
                            indegrees[nxt] -= 1;
                            if indegrees[nxt] == 0 {
                                queue.push_back(nxt);
                            }
                        }
                    }
                    cnt += 1;
                }
            }
            semesters += 1;
        }

        if cnt == n as u16 {
            semesters as i32
        } else {
            -1
        }
    }

    fn build_graph(edges: &Vec<Vec<i32>>, n: usize) -> (HashMap<usize, Vec<usize>>, Vec<u16>) {
        let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut indegrees: Vec<u16> = vec![0; n + 1];
        for edge in edges {
            let from = edge[0] as usize;
            let to = edge[1] as usize;
            graph.entry(from).or_default().push(to);
            indegrees[to] += 1;
        }
        (graph, indegrees)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sample_1_should_return_expected() {
        let n: i32 = 3;
        let relations: Vec<Vec<i32>> = vec![vec![1, 3], vec![2, 3]];
        let actual = Solution::minimum_semesters(n, relations);
        let expected = 2;
        assert_eq!(expected, actual);
    }
}
