/// @author: Leon
/// https://leetcode.com/problems/number-of-increasing-paths-in-a-grid/
/// Time Complexity:    O(`len_rs` * `len_cs`)
/// Space Complexity:   O(`len_rs` * `len_cs`)
/// Reference:
/// https://leetcode.com/problems/number-of-increasing-paths-in-a-grid/discuss/2229819/C%2B%2B-Top-Down-DP
/// https://leetcode.com/problems/number-of-increasing-paths-in-a-grid/discuss/2229827/DFS-%2B-Memo
struct Solution;

#[allow(dead_code)]
impl Solution {
    const DIRS: &'static [isize] = &[0, -1, 0, 1, 0];
    const MOD: i64 = 1e9 as i64 + 7;
    pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let mut memo: Vec<Vec<Option<i64>>> = vec![vec![None; len_cs]; len_rs];
        let mut cnt: i64 = 0;
        for r in 0..len_rs{
            for c in 0..len_cs{
                cnt = (cnt + Self::dfs((r, c), &mut memo, &grid)) % Self::MOD;
            }
        }
        cnt as i32
    }
    fn dfs(coord: (usize, usize), memo: &mut Vec<Vec<Option<i64>>>, grid: &Vec<Vec<i32>>) -> i64{
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let mut cnt: i64 = 1;
        let (r, c) = coord;
        if let Some(m) = memo[r][c]{
            return m;
        }
        for d in 0..4{
            let r_nxt: isize = r as isize + Self::DIRS[d];
            let c_nxt: isize = c as isize + Self::DIRS[d + 1];
            if r_nxt >= 0 && c_nxt >= 0 && (r_nxt as usize) < len_rs && (c_nxt as usize) < len_cs && grid[r_nxt as usize][c_nxt as usize] > grid[r][c]{
                cnt = (cnt + Self::dfs((r_nxt as usize, c_nxt as usize), memo, grid)) % Self::MOD;
            }
        }
        memo[r][c] = Some(cnt);
        cnt
    }
}