use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/course-schedule-ii/
/// Time Complexity:    O(V + E) ~ O(`num_courses` + `_len_pres`) ~ O(`num_courses`)
/// Space Complexity:   O(V + E) ~ O(`num_courses` + `_len_pres`) ~ O(`num_courses`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses: usize = num_courses as usize;
        let mut ans: Vec<i32> = Vec::with_capacity(num_courses);
        let (graph, mut counts) = Self::build_graph(num_courses, prerequisites);
        let mut queue: VecDeque<usize> = {
            let mut queue: VecDeque<usize> = VecDeque::with_capacity(num_courses);
            for (idx, &count) in counts.iter().enumerate() {
                if count == 0 {
                    queue.push_back(idx);
                }
            }
            queue
        };
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some(cur) = queue.pop_front() {
                    ans.push(cur as i32);
                    for &nxt in &graph[cur] {
                        counts[nxt] -= 1;
                        if counts[nxt] == 0 {
                            queue.push_back(nxt);
                        }
                    }
                }
            }
        }
        return if ans.len() == num_courses {
            ans
        } else {
            Vec::new()
        };
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
