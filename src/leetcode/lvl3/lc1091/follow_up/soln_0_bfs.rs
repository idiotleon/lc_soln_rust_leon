use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/shortest-path-in-binary-matrix/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Follow up:
/// to print the shortest path
struct Solution;

#[allow(dead_code)]
impl Solution {
    const DIRS: &'static [(isize, isize)] = &[
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
    ];
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> Vec<(usize, usize)> {
        // sanity check
        if grid[0][0] == 1 {
            return Vec::new();
        }
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let mut queue: VecDeque<(usize, usize)> = VecDeque::with_capacity(len_rs * len_cs);
        queue.push_back((0, 0));
        let mut visited: Vec<Vec<bool>> = vec![vec![false; len_cs]; len_rs];
        let mut _steps: i32 = 0;
        let mut path: Vec<Vec<(usize, usize)>> = vec![vec![(len_rs, len_cs); len_cs]; len_rs];
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some((r_cur, c_cur)) = queue.pop_front() {
                    if r_cur == len_rs - 1 && c_cur == len_cs - 1 {
                        // to construct the path
                        return Self::construct_path((len_rs - 1, len_cs - 1), path);
                    }
                    for (r_delta, c_delta) in Self::DIRS {
                        let r_nxt = r_cur as isize + r_delta;
                        let c_nxt = c_cur as isize + c_delta;
                        if r_nxt < 0 || c_nxt < 0 {
                            continue;
                        }
                        let r_nxt: usize = r_nxt as usize;
                        let c_nxt: usize = c_nxt as usize;
                        if r_nxt >= len_rs
                            || c_nxt >= len_cs
                            || visited[r_nxt][c_nxt]
                            || grid[r_nxt][c_nxt] != 0
                        {
                            continue;
                        }
                        queue.push_back((r_nxt, c_nxt));
                        visited[r_nxt][c_nxt] = true;
                        path[r_nxt][c_nxt] = (r_cur, c_cur);
                    }
                }
            }
            _steps += 1;
        }
        return Vec::new();
    }
    fn construct_path(end: (usize, usize), path: Vec<Vec<(usize, usize)>>) -> Vec<(usize, usize)> {
        let mut ans: Vec<(usize, usize)> = Vec::new();
        let (mut r_cur, mut c_cur) = end;
        loop {
            ans.push((r_cur, c_cur));
            let (r_prev, c_prev) = path[r_cur][c_cur];
            r_cur = r_prev;
            c_cur = c_prev;
            if r_cur == 0 && c_cur == 0 {
                ans.push((r_cur, c_cur));
                break;
            }
        }
        ans.reverse();
        return ans;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let grid: Vec<Vec<i32>> = vec![vec![0, 1], vec![1, 0]];
        let expected: Vec<(usize, usize)> = vec![(0, 0), (1, 1)];
        let actual = Solution::shortest_path_binary_matrix(grid);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let grid: Vec<Vec<i32>> = vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        let expected: Vec<(usize, usize)> = vec![(0, 0), (0, 1), (1, 2), (2, 2)];
        let actual = Solution::shortest_path_binary_matrix(grid);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_3() {
        let grid: Vec<Vec<i32>> = vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]];
        let expected: Vec<(usize, usize)> = Vec::new();
        let actual = Solution::shortest_path_binary_matrix(grid);
        assert_eq!(expected, actual);
    }
}
