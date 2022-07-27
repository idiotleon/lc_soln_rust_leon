use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/equal-row-and-column-pairs/
/// Time Complexity:    O(`len_rs` * `len_cs`)
/// Space Complexity:   O(`len_rs` * ``)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let len_rs: usize = grid.len();
        let len_cs: usize = grid[0].len();
        let row_to_freq: HashMap<&Vec<i32>, u8> = {
            let mut map: HashMap<&Vec<i32>, u8> = HashMap::with_capacity(len_rs);
            for row in &grid {
                *map.entry(row).or_default() += 1;
            }
            map
        };
        let mut cnt: i32 = 0;
        for c in 0..len_cs {
            let mut tmp: Vec<i32> = Vec::with_capacity(len_cs);
            for r in 0..len_rs {
                tmp.push(grid[r][c]);
            }
            if let Some(&freq) = row_to_freq.get(&tmp) {
                cnt += freq as i32;
            }
        }
        cnt
    }
}
