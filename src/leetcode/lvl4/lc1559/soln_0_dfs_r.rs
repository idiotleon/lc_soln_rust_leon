/// @author: Leon
/// https://leetcode.com/problems/detect-cycles-in-2d-grid/
/// Time Complexity:    O(`len_r` * `len_c`)
/// Space Complexity:   O(`len_r` * ``)
struct Solution;

#[allow(dead_code)]
impl Solution {
    const DIRS: &'static [isize] = &[0, -1, 0, 1, 0];
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        const RANGE: usize = 500 + 7;
        let len_r: usize = grid.len();
        let len_c: usize = grid[0].len();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; len_c]; len_r];
        for row in 0..len_r {
            for col in 0..len_c {
                if visited[row][col] {
                    continue;
                }
                if Self::dfs(
                    (row, col),
                    (RANGE, RANGE),
                    &mut visited,
                    grid[row][col],
                    &grid,
                ) {
                    return true;
                }
            }
        }
        false
    }
    fn dfs(
        coord_cur: (usize, usize),
        coord_prev: (usize, usize),
        visited: &mut Vec<Vec<bool>>,
        ch_cur: char,
        grid: &Vec<Vec<char>>,
    ) -> bool {
        let len_r: usize = grid.len();
        let len_c: usize = grid[0].len();
        let (r_cur, c_cur) = coord_cur;
        let (r_prev, c_prev) = coord_prev;
        for d in 0..4 {
            let r_nxt = r_cur as isize + Self::DIRS[d];
            let c_nxt = c_cur as isize + Self::DIRS[d + 1];
            if r_nxt < 0
                || c_nxt < 0
                || r_nxt as usize >= len_r
                || c_nxt as usize >= len_c
                || grid[r_nxt as usize][c_nxt as usize] != ch_cur
            {
                continue;
            }
            let r_nxt: usize = r_nxt as usize;
            let c_nxt: usize = c_nxt as usize;
            if r_nxt == r_prev && c_nxt == c_prev {
                continue;
            }
            if visited[r_nxt][c_nxt] {
                return true;
            }
            visited[r_nxt][c_nxt] = true;
            if Self::dfs((r_nxt, c_nxt), (r_cur, c_cur), visited, ch_cur, grid) {
                return true;
            };
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
