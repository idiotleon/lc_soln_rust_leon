use std::collections::VecDeque;
/// @author: Leon
/// https://leetcode.com/problems/number-of-islands/
/// Time Complexity:    O(`len_r` * `len_c`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    const DIRS: &'static [isize] = &[0, -1, 0, 1, 0];
    const LAND: char = '1';
    const WATER: char = '0';
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let len_r: usize = grid.len();
        let len_c: usize = grid[0].len();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; len_c]; len_r];
        let mut cnt: i32 = 0;
        for r in 0..len_r {
            for c in 0..len_c {
                if grid[r][c] == Self::LAND && !visited[r][c] {
                    visited[r][c] = true;
                    Self::bfs((r, c), &mut visited, &grid);
                    cnt += 1;
                }
            }
        }
        cnt
    }
    fn bfs(coord: (usize, usize), visited: &mut Vec<Vec<bool>>, grid: &Vec<Vec<char>>) {
        let len_r: usize = grid.len();
        let len_c: usize = grid[0].len();
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        queue.push_back(coord);
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some((r_cur, c_cur)) = queue.pop_front() {
                    for d in 0..4 {
                        let r_nxt: isize = r_cur as isize + Self::DIRS[d];
                        let c_nxt: isize = c_cur as isize + Self::DIRS[d + 1];
                        if r_nxt < 0
                            || c_nxt < 0
                            || r_nxt as usize >= len_r
                            || c_nxt as usize >= len_c
                        {
                            continue;
                        }
                        let r_nxt: usize = r_nxt as usize;
                        let c_nxt: usize = c_nxt as usize;
                        if grid[r_nxt][c_nxt] == Self::WATER || visited[r_nxt][c_nxt] {
                            continue;
                        }
                        visited[r_nxt][c_nxt] = true;
                        queue.push_back((r_nxt, c_nxt));
                    }
                }
            }
        }
    }
}
