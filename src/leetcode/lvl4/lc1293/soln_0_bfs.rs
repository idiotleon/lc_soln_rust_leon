use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/shortest-path-in-a-grid-with-obstacles-elimination/
/// Time Complexity:    O(`len_rs` * `len_cs`)
/// Space Complexity:   O(`len_rs` * `len_cs`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        const DIRS: &[isize] = &[0, -1, 0, 1, 0];
        const EMPTY: i32 = 0;
        const OBSTACLE: i32 = 1;
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let mut seen: Vec<Vec<Option<i32>>> = vec![vec![None; len_cs]; len_rs];
        let mut queue: VecDeque<(isize, isize, i32)> = {
            let mut queue = VecDeque::with_capacity(len_rs * len_cs);
            queue.push_back((0, 0, k));
            queue
        };
        let mut steps: i32 = 0;
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some((r, c, k_cur)) = queue.pop_front() {
                    if r == len_rs as isize - 1 && c == len_cs as isize - 1 {
                        return steps;
                    }
                    for d in 0..4 {
                        let r_nxt: isize = r + DIRS[d];
                        let c_nxt: isize = c + DIRS[d + 1];
                        if r_nxt < 0
                            || c_nxt < 0
                            || r_nxt as usize >= len_rs
                            || c_nxt as usize >= len_cs
                        {
                            continue;
                        }
                        let k_nxt = if grid[r_nxt as usize][c_nxt as usize] == OBSTACLE {
                            k_cur - 1
                        } else {
                            k_cur
                        };
                        if k_nxt < 0 || seen[r_nxt as usize][c_nxt as usize].unwrap_or(-1) >= k_nxt
                        {
                            continue;
                        }
                        seen[r_nxt as usize][c_nxt as usize] = Some(k_nxt);
                        queue.push_back((r_nxt, c_nxt, k_nxt));
                    }
                }
            }
            steps += 1;
        }
        return -1;
    }
}
