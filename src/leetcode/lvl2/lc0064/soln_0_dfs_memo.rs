/// @author: Leon
/// https://leetcode.com/problems/minimum-path-sum/
/// Time Complexity:    O(`len_r` * `len_c`)
/// Space Complexity:   O(`len_r` * `len_c`)
/// Reference:
/// https://leetcode.com/problems/minimum-path-sum/discuss/185358/Easy-Recursive-Solution-with-memo-(Java)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let len_r: usize = grid.len();
        let len_c: usize = grid[0].len();
        let mut memo: Vec<Vec<Option<i32>>> = vec![vec![None; len_c]; len_r];
        Self::dfs((0, 0), &grid, &mut memo)
    }
    fn dfs(coord: (isize, isize), grid: &Vec<Vec<i32>>, memo: &mut Vec<Vec<Option<i32>>>) -> i32 {
        let len_r: usize = grid.len();
        let len_c: usize = grid[0].len();
        let (r, c) = coord;
        if r < 0 || c < 0 || r as usize >= len_r || c as usize >= len_c {
            return i32::MAX;
        }
        let r: usize = r as usize;
        let c: usize = c as usize;
        if let Some(m) = memo[r][c] {
            return m;
        }
        if r == len_r - 1 && c == len_c - 1 {
            return grid[r][c];
        }
        let min_path_sum = grid[r][c]
            + std::cmp::min(
                Self::dfs((r as isize + 1, c as isize), grid, memo),
                Self::dfs((r as isize, c as isize + 1), grid, memo),
            );
        memo[r][c] = Some(min_path_sum);
        min_path_sum
    }
}
