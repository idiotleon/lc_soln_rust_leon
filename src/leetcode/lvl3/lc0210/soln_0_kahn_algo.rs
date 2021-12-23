/// @author: Leon
/// https://leetcode.com/problems/course-schedule-ii/
/// Time Complexity:    O(`num_courses` + `_n_pres`)
/// Space Complexity:   O(`num_courses` + `n_pres`)
use std::collections::VecDeque;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_order(num_courses: u16, prerequisites: Vec<Vec<u16>>) -> Vec<u16> {
        let _n_pres = prerequisites.len();
        let (graph, mut indegrees) = Self::build_graph(num_courses, prerequisites);
        let mut queue: VecDeque<usize> = VecDeque::new();
        for (idx, &indegree) in indegrees.iter().enumerate() {
            if indegree == 0 {
                queue.push_back(idx);
            }
        }
        let mut idx: usize = 0;
        let ans: Vec<u16> = {
            let mut ans: Vec<u16> = vec![0; num_courses as usize];
            while !queue.is_empty() {
                let cur = queue.pop_front().unwrap();
                ans[idx] = cur as u16;
                idx += 1;
                for &prev in graph[cur].iter() {
                    indegrees[prev as usize] -= 1;
                    if indegrees[prev as usize] == 0 {
                        queue.push_back(prev as usize);
                    }
                }
            }
            ans
        };
        if idx < num_courses as usize {
            return vec![];
        }
        ans
    }
    fn build_graph(num_courses: u16, prerequisites: Vec<Vec<u16>>) -> (Vec<Vec<u16>>, Vec<u16>) {
        let mut indegrees: Vec<u16> = vec![0; num_courses as usize];
        let mut graph: Vec<Vec<u16>> = vec![vec![]; num_courses as usize];
        for pres in prerequisites {
            let cur = pres[0];
            let pre = pres[1];
            graph[pre as usize].push(cur);
            indegrees[cur as usize] += 1;
        }
        (graph, indegrees)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sample_input_1() {
        let num_courses: u16 = 2;
        let prerequisites: Vec<Vec<u16>> = vec![vec![1, 0]];
        let actual = Solution::find_order(num_courses, prerequisites);
        let expected: Vec<Vec<u16>> = vec![vec![0, 1]];
        assert!(true == expected.contains(&actual));
    }
    #[test]
    fn test_sample_input_2() {
        let num_courses: u16 = 4;
        let prerequisites: Vec<Vec<u16>> = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
        let actual: Vec<u16> = Solution::find_order(num_courses, prerequisites);
        let expected: Vec<Vec<u16>> = vec![vec![0, 2, 1, 3], vec![0, 1, 2, 3], vec![0, 2, 1, 3]];
        assert!(true == expected.contains(&actual));
    }
}
