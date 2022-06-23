/// @author: Leon
/// https://leetcode.com/problems/max-area-of-island/
/// Time Complexity:    O(`len_rs` * `len_cs`)
/// Space Complexity:   O(`len_rs` * `len_cs`)
/// Reference:
/// https://leetcode.com/problems/max-area-of-island/discuss/889283/Rust-DFS-4ms-72
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        const DIRS: [isize; 5] = [0, -1, 0, 1, 0];
        fn dfs(r: isize, c: isize, visited: &mut Vec<Vec<bool>>, grid: &Vec<Vec<i32>>) -> i32 {
            let len_rs: usize = grid.len();
            let len_cs: usize = grid[0].len();
            if r < 0
                || r >= len_rs as isize
                || c < 0
                || c >= len_cs as isize
                || visited[r as usize][c as usize]
                || grid[r as usize][c as usize] == 0
            {
                return 0;
            }
            let mut area: i32 = 1;
            visited[r as usize][c as usize] = true;
            for d in 0..4 as usize {
                let (nxt_r, nxt_c) = (r + DIRS[d], c + DIRS[d + 1]);

                area += dfs(nxt_r, nxt_c, visited, grid);
            }
            area
        }
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let mut max_area: i32 = 0;
        let mut visited: Vec<Vec<bool>> = vec![vec![false; len_cs]; len_rs];
        for r in 0..len_rs {
            for c in 0..len_cs {
                if grid[r][c] == 0 {
                    continue;
                }

                let cur_area = dfs(r as isize, c as isize, &mut visited, &grid);
                max_area = std::cmp::max(max_area, cur_area);
            }
        }
        max_area
    }
}
