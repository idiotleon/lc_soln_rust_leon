use std::collections::VecDeque;
/// @author: Leon
/// https://leetcode.com/problems/course-schedule-ii/
/// Time Complexity:    O(`num_courses` + `_len_pres`)
/// Space Complexity:   O(`num_courses` + `_len_pres`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let len_cs: usize = num_courses as usize;
        let _len_pres = prerequisites.len();
        let (graph, mut indegrees) = Self::build_graph(len_cs, prerequisites);
        let mut queue: VecDeque<usize> = VecDeque::new();
        for (idx, &indegree) in indegrees.iter().enumerate() {
            if indegree == 0 {
                queue.push_back(idx);
            }
        }
        let mut ans: Vec<i32> = Vec::with_capacity(len_cs);
        while !queue.is_empty() {
            if let Some(cur) = queue.pop_front() {
                ans.push(cur as i32);
                for &nxt in graph[cur].iter() {
                    indegrees[nxt] -= 1;
                    if indegrees[nxt] == 0 {
                        queue.push_back(nxt);
                    }
                }
            }
        }
        if ans.len() < len_cs {
            return vec![];
        }
        ans
    }
    fn build_graph(let_vts: usize, prerequisites: Vec<Vec<i32>>) -> (Vec<Vec<usize>>, Vec<u16>) {
        let mut indegrees: Vec<u16> = vec![0; let_vts];
        let mut graph: Vec<Vec<usize>> = vec![vec![]; let_vts];
        for pres in prerequisites {
            let cur: usize = pres[0] as usize;
            let pre: usize = pres[1] as usize;
            graph[pre].push(cur);
            indegrees[cur] += 1;
        }
        (graph, indegrees)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sample_input_1() {
        let num_courses: i32 = 2;
        let prerequisites: Vec<Vec<i32>> = vec![vec![1, 0]];
        let actual = Solution::find_order(num_courses, prerequisites);
        let expected: Vec<Vec<i32>> = vec![vec![0, 1]];
        assert!(expected.contains(&actual));
    }
    #[test]
    fn test_sample_input_2() {
        let num_courses: i32 = 4;
        let prerequisites: Vec<Vec<i32>> = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
        let actual = Solution::find_order(num_courses, prerequisites);
        let expected: Vec<Vec<i32>> = vec![vec![0, 2, 1, 3], vec![0, 1, 2, 3], vec![0, 2, 1, 3]];
        assert!(expected.contains(&actual));
    }
}
