use std::cmp::min;

/// @author: Leon
/// https://leetcode.com/problems/paint-house/
/// Time Complexity:    O(`_len_hs`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/paint-house/discuss/68211/Simple-java-DP-solution/70111
/// https://leetcode.com/problems/paint-house/discuss/68211/Simple-java-DP-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        let _len_hs: usize = costs.len();
        let mut prev0: i32 = 0;
        let mut prev1: i32 = 0;
        let mut prev2: i32 = 0;
        for cost in costs {
            let cur0 = cost[0] + min(prev1, prev2);
            let cur1 = cost[1] + min(prev0, prev2);
            let cur2 = cost[2] + min(prev0, prev1);
            prev0 = cur0;
            prev1 = cur1;
            prev2 = cur2;
        }
        vec![prev0, prev1, prev2].into_iter().min().unwrap()
    }
}
