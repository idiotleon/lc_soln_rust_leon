use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/detect-cycles-in-2d-grid/
/// Time Complexity:    O(`len_r` * `len_c`)
/// Space Complexity:   O(`len_r` * `len_c`)
/// !!! This is not correct solution !!!
/// Reference:
/// https://leetcode.com/problems/detect-cycles-in-2d-grid/discuss/805673/C++-BFS/1068703
/// https://leetcode.com/problems/detect-cycles-in-2d-grid/discuss/805673/C%2B%2B-BFS
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        const DIRS: &'static [isize] = &[0, -1, 0, 1, 0];
        let len_r: usize = grid.len();
        let len_c: usize = grid[0].len();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; len_c]; len_r];
        for r in 0..len_r {
            for c in 0..len_c {
                if visited[r][c] {
                    continue;
                }
                let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
                queue.push_back((r, c));
                while !queue.is_empty() {
                    let len_q: usize = queue.len();
                    for _ in 0..len_q {
                        let (r_cur, c_cur) = queue.pop_front().unwrap();
                        if visited[r_cur][c_cur] {
                            return true;
                        }
                        visited[r_cur][c_cur] = true;
                        for d in 0..4 {
                            let r_nxt = r_cur as isize + DIRS[d];
                            let c_nxt = c_cur as isize + DIRS[d + 1];
                            if r_nxt < 0
                                || c_nxt < 0
                                || r_nxt as usize >= len_r
                                || c_nxt as usize >= len_c
                                || grid[r_nxt as usize][c_nxt as usize] != grid[r_cur][c_cur]
                            {
                                continue;
                            }
                            queue.push_back((r_nxt as usize, c_nxt as usize));
                        }
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let grid: Vec<Vec<char>> = vec![
            vec!['a', 'a', 'a', 'a'],
            vec!['a', 'b', 'b', 'a'],
            vec!['a', 'b', 'b', 'a'],
            vec!['a', 'a', 'a', 'a'],
        ];
        let actual = Solution::contains_cycle(grid);
        let expected = true;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let grid: Vec<Vec<char>> = vec![
            vec!['c', 'c', 'c', 'a'],
            vec!['c', 'd', 'c', 'c'],
            vec!['c', 'c', 'e', 'c'],
            vec!['f', 'c', 'c', 'c'],
        ];
        let actual = Solution::contains_cycle(grid);
        let expected = true;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_test_case_27() {
        let grid: Vec<Vec<char>> = vec![
            vec!['a', 'a', 'b'],
            vec!['b', 'z', 'b'],
            vec!['b', 'b', 'a'],
        ];
        let actual = Solution::contains_cycle(grid);
        let expected = false;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_test_case_40() {
        let grid: Vec<Vec<char>> = vec![
            vec!['d', 'b', 'b'],
            vec!['c', 'a', 'a'],
            vec!['b', 'a', 'c'],
            vec!['c', 'c', 'c'],
            vec!['d', 'd', 'a'],
        ];
        let actual = Solution::contains_cycle(grid);
        let expected = false;
        assert_eq!(expected, actual);
    }
}
