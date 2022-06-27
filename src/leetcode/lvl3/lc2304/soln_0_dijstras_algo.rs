use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// @author: Leon
/// https://leetcode.com/problems/minimum-path-cost-in-a-grid/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/minimum-path-cost-in-a-grid/discuss/2141640/C%2B%2B-Dijkstra-Algorithm-or-Short-overkill-but-100-runtime
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_path_cost(grid: Vec<Vec<i32>>, move_costs: Vec<Vec<i32>>) -> i32 {
        let len_rs: usize = grid.len();
        let len_vts: usize = grid[0].len();
        const RANGE: i32 = 50 * 100 + 7;
        let mut costs: Vec<Vec<i32>> = vec![vec![RANGE; len_vts]; len_rs];
        // min heap
        let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = {
            let mut heap = BinaryHeap::new();
            for c in 0..len_vts {
                heap.push(Reverse((grid[0][c], 0, c)));
            }
            heap
        };
        while let Some(Reverse((cost_cur, r, c))) = heap.pop() {
            if r == len_rs - 1 {
                return cost_cur;
            }
            'inner: for idx_c in 0..len_vts {
                if r + 1 == len_rs {
                    break 'inner;
                }
                let cost_new =
                    cost_cur + move_costs[grid[r][c] as usize][idx_c] + grid[1 + r][idx_c];
                if cost_new < costs[1 + r][idx_c] {
                    costs[1 + r][idx_c] = cost_new;
                    heap.push(Reverse((cost_new, 1 + r, idx_c)));
                }
            }
        }
        unreachable!()
    }
}
