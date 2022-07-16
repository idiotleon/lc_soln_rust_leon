/// @author: Leon
/// https://leetcode.com/problems/out-of-boundary-paths/
/// Time Complexity:    O(`m` * `n` * `max_move`)
/// Space Complexity:   O(`m` * `n` * `max_move`)
/// Reference:
/// https://leetcode.com/problems/out-of-boundary-paths/discuss/1294732/C%2B%2BJavaPython-DFS-and-memoization-Clean-and-Concise
struct Solution;

#[allow(dead_code)]
impl Solution {
    const DIRS: &'static [isize] = &[0, -1, 0, 1, 0];
    const MOD: i32 = 1e9 as i32 + 7;
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let len_rs: usize = m as usize;
        let len_cs: usize = n as usize;
        let coord_start: (isize, isize) = (start_row as isize, start_column as isize);
        let mut memo: Vec<Vec<Vec<Option<i32>>>> =
            vec![vec![vec![None; 1 + len_cs]; 1 + len_rs]; 1 + max_move as usize];
        Self::dfs(
            coord_start,
            max_move as usize,
            len_rs as isize,
            len_cs as isize,
            &mut memo,
        )
    }
    fn dfs(
        coord: (isize, isize),
        max_move: usize,
        len_rs: isize,
        len_cs: isize,
        memo: &mut Vec<Vec<Vec<Option<i32>>>>,
    ) -> i32 {
        let (r, c) = coord;
        if r < 0 || c < 0 || r >= len_rs || c >= len_cs {
            return 1;
        }
        if max_move == 0 {
            return 0;
        }
        if let Some(m) = memo[max_move][r as usize][c as usize] {
            return m;
        }
        let mut res: i32 = 0;
        for d in 0..4 {
            let r_nxt: isize = r + Self::DIRS[d];
            let c_nxt: isize = c + Self::DIRS[d + 1];
            res = (res + Self::dfs((r_nxt, c_nxt), max_move - 1, len_rs, len_cs, memo)) % Self::MOD;
        }
        memo[max_move][r as usize][c as usize] = Some(res);
        res
    }
}
