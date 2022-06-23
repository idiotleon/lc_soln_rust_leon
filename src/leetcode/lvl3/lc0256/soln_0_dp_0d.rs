use std::cmp::min;
/// @author: Leon
/// https://leetcode.com/problems/paint-house/
/// Time Complexity:    O(`_len_cts`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/paint-house/discuss/68211/Simple-java-DP-solution/70111
/// https://leetcode.com/problems/paint-house/discuss/68211/Simple-java-DP-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        let _len_cts: usize = costs.len();
        let mut r: i32 = 0;
        let mut b: i32 = 0;
        let mut g: i32 = 0;
        for cost in costs.into_iter() {
            let (prev_r, prev_b, prev_g) = (r, b, g);
            r = cost[0] + min(prev_b, prev_g);
            b = cost[1] + min(prev_r, prev_g);
            g = cost[2] + min(prev_r, prev_b);
        }
        min(r, min(g, b))
    }
}
