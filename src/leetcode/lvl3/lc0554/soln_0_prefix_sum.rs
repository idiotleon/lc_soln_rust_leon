use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/brick-wall/
/// Time Complexity:    O(`len_rs` * avg_len_cs)
/// Space Complexity:   O(M)
/// Reference:
/// https://leetcode.com/problems/brick-wall/editorial/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let len_rs: usize = wall.len();
        let sum_to_freq: HashMap<i32, i32> = {
            let mut map: HashMap<i32, i32> = HashMap::new();
            for row in wall {
                let mut sum: i32 = 0;
                let len_cs: usize = row.len();
                for c in 0..len_cs - 1 {
                    sum += row[c];
                    *map.entry(sum).or_default() += 1;
                }
            }
            map
        };
        let mut ans: i32 = len_rs as i32;
        for (_sum, freq) in sum_to_freq.into_iter() {
            ans = std::cmp::min(ans, len_rs as i32 - freq);
        }
        return ans;
    }
}
