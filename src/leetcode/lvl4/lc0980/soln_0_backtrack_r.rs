/// @author: Leon
/// https://leetcode.com/problems/unique-paths-iii/description/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/unique-paths-iii/discuss/289954/Java-Solution-DFS-With-backtracking
struct Solution;

#[allow(dead_code)]
impl Solution {
    const DIRS: &'static [isize] = &[0, -1, 0, 1, 0];
    const EMPTY: i32 = 0;
    const OBSTACLE: i32 = -1;
    const SQUARE_ENDING: i32 = 2;
    const SQUARE_STARTING: i32 = 1;
    pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let (r_start, c_start, cnt_empty) = {
            let mut r_start: usize = 0;
            let mut c_start: usize = 0;
            let mut cnt_empty: i32 = 0;
            for r in 0..len_rs {
                for c in 0..len_cs {
                    if grid[r][c] == 0 {
                        cnt_empty += 1;
                    } else if grid[r][c] == Self::SQUARE_STARTING {
                        r_start = r;
                        c_start = c;
                    }
                }
            }
            (r_start, c_start, cnt_empty)
        };
        return Self::backtrack(
            (r_start as isize, c_start as isize),
            Self::OBSTACLE,
            cnt_empty,
            &mut grid,
        );
    }
    fn backtrack(coord: (isize, isize), cnt: i32, need: i32, grid: &mut Vec<Vec<i32>>) -> i32 {
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let (r, c) = coord;
        if r < 0
            || c < 0
            || r as usize >= len_rs
            || c as usize >= len_cs
            || grid[r as usize][c as usize] == Self::OBSTACLE
        {
            return 0;
        }
        if grid[r as usize][c as usize] == Self::SQUARE_ENDING {
            if cnt == need {
                return 1;
            } else {
                return 0;
            }
        }
        grid[r as usize][c as usize] = Self::OBSTACLE;
        let mut total: i32 = 0;
        for d in 0..4 {
            let r_nxt = r + Self::DIRS[d];
            let c_nxt = c + Self::DIRS[d + 1];
            total += Self::backtrack((r_nxt, c_nxt), cnt + 1, need, grid);
        }
        grid[r as usize][c as usize] = 0;
        return total;
    }
}
