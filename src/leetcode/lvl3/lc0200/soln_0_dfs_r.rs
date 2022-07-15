/// @author: Leon
/// https://leetcode.com/problems/number-of-islands/
/// Time Complexity:    O(`len_rs` * `len_cs`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    const DIRS: &'static [isize] = &[0, -1, 0, 1, 0];
    const LAND: char = '1';
    const WATER: char = '0';
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; len_cs]; len_rs];
        let mut cnt: i32 = 0;
        for r in 0..len_rs {
            for c in 0..len_cs {
                if grid[r][c] == Self::LAND && !visited[r][c] {
                    cnt += 1;
                    Self::dfs((r, c), &mut visited, &grid);
                }
            }
        }
        cnt
    }
    fn dfs(coord: (usize, usize), visited: &mut Vec<Vec<bool>>, grid: &Vec<Vec<char>>) {
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let (r_cur, c_cur) = coord;
        visited[r_cur][c_cur] = true;
        for d in 0..4 {
            let r_nxt: isize = r_cur as isize + Self::DIRS[d];
            let c_nxt: isize = c_cur as isize + Self::DIRS[d + 1];
            if r_nxt < 0 || c_nxt < 0 || r_nxt as usize >= len_rs || c_nxt as usize >= len_cs {
                continue;
            }
            let r_nxt: usize = r_nxt as usize;
            let c_nxt: usize = c_nxt as usize;
            if grid[r_nxt][c_nxt] == Self::WATER || visited[r_nxt][c_nxt] {
                continue;
            }
            Self::dfs((r_nxt, c_nxt), visited, grid);
        }
    }
}
