use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/max-area-of-island/
/// Time Complexity:    O(`len_rs` * `len_cs`)
/// Space Complexity:   O(`len_rs` * `len_cs`) / O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    const DIRS: &'static [isize] = &[0, -1, 0, 1, 0];
    const ISLAND: i32 = 1;
    const WATER: i32 = 0;
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let mut visited: Vec<Vec<bool>> = vec![vec![false; len_cs]; len_rs];
        let mut area_max: i32 = 0;
        for r in 0..len_rs {
            for c in 0..len_cs {
                if grid[r][c] == Self::ISLAND {
                    if visited[r][c] {
                        continue;
                    }
                    let area_cur: i32 = Self::bfs((r as isize, c as isize), &mut visited, &grid);
                    area_max = std::cmp::max(area_max, area_cur);
                }
            }
        }
        area_max
    }
    fn bfs(coord: (isize, isize), visited: &mut Vec<Vec<bool>>, grid: &Vec<Vec<i32>>) -> i32 {
        let len_rs: isize = grid.len() as isize;
        let len_cs: isize = grid[0].len() as isize;
        let (r, c) = coord;
        let mut queue: VecDeque<(isize, isize)> = VecDeque::new();
        queue.push_back((r, c));
        let mut area: i32 = 1;
        visited[r as usize][c as usize] = true;
        while !queue.is_empty() {
            let len_q: usize = queue.len();
            for _ in 0..len_q {
                if let Some((r, c)) = queue.pop_front() {
                    for d in 0..4 {
                        let r_nxt: isize = r + Self::DIRS[d];
                        let c_nxt: isize = c + Self::DIRS[d + 1];
                        if r_nxt < 0
                            || c_nxt < 0
                            || r_nxt >= len_rs
                            || c_nxt >= len_cs
                            || grid[r_nxt as usize][c_nxt as usize] == Self::WATER
                            || visited[r_nxt as usize][c_nxt as usize]
                        {
                            continue;
                        }
                        area += 1;
                        visited[r_nxt as usize][c_nxt as usize] = true;
                        queue.push_back((r_nxt, c_nxt));
                    }
                }
            }
        }
        area
    }
}
