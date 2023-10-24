use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/detect-cycles-in-2d-grid/
/// Time Complexity:    O(`len_rs` * `len_cs`)
/// Space Complexity:   O(`len_rs` * `len_cs`)
/// Reference:
/// https://leetcode.com/problems/detect-cycles-in-2d-grid/discuss/805673/C++-BFS/1068703
/// https://leetcode.com/problems/detect-cycles-in-2d-grid/discuss/805673/C%2B%2B-BFS
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        const DIRS: &'static [isize] = &[0, -1, 0, 1, 0];
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; len_cs]; len_rs];
        for r in 0..len_rs {
            for c in 0..len_cs {
                if visited[r][c] {
                    continue;
                }
                let mut queue: VecDeque<(usize, usize)> = VecDeque::with_capacity(len_rs * len_cs);
                queue.push_back((r, c));
                while !queue.is_empty() {
                    let len_q: usize = queue.len();
                    for _ in 0..len_q {
                        if let Some((r_cur, c_cur)) = queue.pop_front() {
                            if visited[r_cur][c_cur] {
                                return true;
                            }
                            visited[r_cur][c_cur] = true;
                            for d in 0..4 {
                                let r_nxt = r_cur as isize + DIRS[d];
                                let c_nxt = c_cur as isize + DIRS[d + 1];
                                if r_nxt < 0 || c_nxt < 0 {
                                    continue;
                                }
                                let r_nxt: usize = r_nxt as usize;
                                let c_nxt: usize = c_nxt as usize;
                                if r_nxt >= len_rs
                                    || c_nxt >= len_cs
                                    || grid[r_nxt][c_nxt] != grid[r_cur][c_cur]
                                    || visited[r_nxt][c_nxt]
                                {
                                    continue;
                                }
                                queue.push_back((r_nxt as usize, c_nxt as usize));
                            }
                        }
                    }
                }
            }
        }
        return false;
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
