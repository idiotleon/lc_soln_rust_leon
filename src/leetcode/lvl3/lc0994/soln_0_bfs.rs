use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/rotting-oranges/
/// Time Complexity:    O(`len_r` * `len_c`)
/// Space Complexity:   O(max(`len_r`, `len_c`))
struct Solution;

#[allow(dead_code)]
impl Solution {
    const DIRS: &'static [i8] = &[0, -1, 0, 1, 0];
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let len_r = grid.len();
        let len_c = grid[0].len();
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
        if cnt_fresh == (len_r * len_c) as i32 {
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
                        let row_nxt = row as i8 + Self::DIRS[d];
                        let col_nxt = col as i8 + Self::DIRS[d + 1];
                        if row_nxt < 0
                            || row_nxt as usize >= len_r
                            || col_nxt < 0
                            || col_nxt as usize >= len_c
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
