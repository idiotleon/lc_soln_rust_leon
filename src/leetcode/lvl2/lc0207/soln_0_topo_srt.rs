use std::collections::VecDeque;
/// @author: Leon
/// https://leetcode.com/problems/course-schedule/
/// Time Complexity:    O(V + E) ~ O(`num_courses` + `_len_pres`)
/// Space Complexity:   O(V + E) ~ O(`num_courses` + `_len_pres`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let _len_pres: usize = prerequisites.len();
        let (graph, mut indegrees) = Self::build_graph(num_courses as usize, &prerequisites);
        let mut queue: VecDeque<usize> = VecDeque::new();
        for (idx, &degree) in indegrees.iter().enumerate() {
            if degree == 0 {
                queue.push_back(idx);
            }
        }
        let mut cnt = 0;
        while let Some(cur) = queue.pop_front() {
            cnt += 1;
            if let Some(nxts) = graph.get(cur) {
                for &nxt in nxts {
                    indegrees[nxt] -= 1;
                    if indegrees[nxt] == 0 {
                        queue.push_back(nxt);
                    }
                }
            };
        }
        cnt == num_courses
    }
    fn build_graph(n: usize, pres: &Vec<Vec<i32>>) -> (Vec<Vec<usize>>, Vec<i32>) {
        let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut indegrees: Vec<i32> = vec![0; n];
        for edge in pres.iter() {
            let ready = edge[0];
            let pre = edge[1];
            graph[pre as usize].push(ready as usize);
            indegrees[ready as usize] += 1;
        }
        (graph, indegrees)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0]];
        let actual = Solution::can_finish(num_courses, prerequisites);
        let expected = true;
        assert_eq!(expected, actual);
    }
}