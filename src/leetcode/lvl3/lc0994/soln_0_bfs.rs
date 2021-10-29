/// https://leetcode.com/problems/rotting-oranges/
///
/// Time Complexity:    O(`n_rows` * `n_cols`)
/// Space Complexity:   O(max(`n_rows`, `n_cols`))
use std::collections::VecDeque;

#[allow(dead_code)]
struct Solution;

static DIRS: &'static [i8] = &[0, -1, 0, 1, 0];

#[allow(dead_code)]
impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let n_rows = grid.len();
        let n_cols = grid[0].len();
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        let cnt_fresh = {
            let mut cnt_tmp = 0;
            for (row, nums) in grid.iter().enumerate() {
                for (col, &state) in nums.iter().enumerate() {
                    match state {
                        state if state == State::ROTTEN as i32 => {
                            queue.push_back((row, col));
                        }
                        state if state == State::FRESH as i32 => {
                            cnt_tmp += 1;
                        }
                        _ => {}
                    }
                }
            }
            cnt_tmp
        };
        if cnt_fresh == 0 {
            return 0;
        };
        if cnt_fresh == (n_rows * n_cols) as i32 {
            return -1;
        }
        let mut lvl = 0;
        let mut cnt_rotten = 0;
        let mut grid = grid;
        while !queue.is_empty() {
            let len = queue.len();
            lvl += 1;
            for _ in 0..len {
                if let Some((row, col)) = queue.pop_front() {
                    for d in 0..4 {
                        let row_nxt = row as i8 + DIRS[d];
                        let col_nxt = col as i8 + DIRS[d + 1];
                        if row_nxt < 0
                            || row_nxt as usize >= n_rows
                            || col_nxt < 0
                            || col_nxt as usize >= n_cols
                            || grid[row_nxt as usize][col_nxt as usize] != State::FRESH as i32
                        {
                            continue;
                        }
                        let row_nxt = row_nxt as usize;
                        let col_nxt = col_nxt as usize;
                        grid[row_nxt][col_nxt] = State::ROTTEN as i32;
                        queue.push_back((row_nxt, col_nxt));
                        cnt_rotten += 1;
                        if cnt_fresh == cnt_rotten {
                            return lvl;
                        }
                    }
                }
            }
        }
        -1
    }
}
enum State {
    _EMPTY = 0,
    FRESH = 1,
    ROTTEN = 2,
}
