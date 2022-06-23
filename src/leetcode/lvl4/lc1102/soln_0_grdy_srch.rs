use std::collections::BinaryHeap;

/// @author: Leon
/// https://leetcode.com/problems/path-with-maximum-minimum-value/
/// Time Complexity:    O(V * lg(V)) ~ O(`len_r` * `len_c` * lg(`len_r` * `len_c`))
/// Space Complexity:   O(V) ~ O(`len_r` * `len_c`)
/// Reference:
/// https://leetcode.com/problems/path-with-maximum-minimum-value/discuss/324923/Clear-Code-Dijkstra-Algorithm-(C%2B%2BJavaPythonGoPHP)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_minimum_path(mut grid: Vec<Vec<i32>>) -> i32 {
        let len_r: usize = grid.len();
        let len_c: usize = grid[0].len();
        const DIRS: &[isize] = &[0, -1, 0, 1, 0];
        const VISITED: i32 = -1;
        let mut heap = {
            let mut heap: BinaryHeap<(i32, usize, usize)> = BinaryHeap::new();
            heap.push((grid[0][0], 0, 0));
            heap
        };
        let mut most: i32 = grid[0][0];
        grid[0][0] = VISITED;
        while let Some((score, r, c)) = heap.pop() {
            most = std::cmp::min(most, score);
            if r == len_r - 1 && c == len_c - 1 {
                break;
            }
            for d in 0..4 {
                let nxt_r: isize = r as isize + DIRS[d];
                let nxt_c: isize = c as isize + DIRS[d + 1];
                if nxt_r < 0
                    || nxt_c < 0
                    || nxt_r as usize >= len_r
                    || nxt_c as usize >= len_c
                    || grid[nxt_r as usize][nxt_c as usize] < 0
                {
                    continue;
                }
                let nxt_r: usize = nxt_r as usize;
                let nxt_c: usize = nxt_c as usize;
                heap.push((grid[nxt_r][nxt_c], nxt_r, nxt_c));
                grid[nxt_r][nxt_c] = VISITED;
            }
        }
        most
    }
}
