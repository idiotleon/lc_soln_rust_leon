use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/course-schedule/
/// Time Complexity:    O(V + E) ~ O(`num_courses` + `_len_pres`)
/// Space Complexity:   O(V + E) ~ O(`num_courses` + `_len_pres`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let len_ps: usize = prerequisites.len();
        let (graph, mut counts) = Self::build_graph(num_courses as usize, prerequisites);
        let mut queue: VecDeque<usize> = VecDeque::with_capacity(len_ps);
        for (idx, &count) in counts.iter().enumerate() {
            if count == 0 {
                queue.push_back(idx);
            }
        }
        let mut cnt: u16 = 0;
        while let Some(cur) = queue.pop_front() {
            cnt += 1;
            for &nxt in &graph[cur] {
                counts[nxt] -= 1;
                if counts[nxt] == 0 {
                    queue.push_back(nxt);
                }
            }
        }
        return cnt as i32 == num_courses as i32;
    }
    fn build_graph(
        num_courses: usize,
        prerequisites: Vec<Vec<i32>>,
    ) -> (Vec<Vec<usize>>, Vec<u16>) {
        let mut graph: Vec<Vec<usize>> = vec![Vec::with_capacity(num_courses); num_courses];
        let mut counts: Vec<u16> = vec![0; num_courses];
        for pre in prerequisites {
            let to: usize = pre[0] as usize;
            let from: usize = pre[1] as usize;
            graph[from].push(to);
            counts[to] += 1;
        }
        return (graph, counts);
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
    #[test]
    fn it_works_with_sample_input_2() {
        let num_courses = 2;
        let prerequisites = vec![vec![1, 0], vec![0, 1]];
        let actual = Solution::can_finish(num_courses, prerequisites);
        let expected = false;
        assert_eq!(expected, actual);
    }
}
