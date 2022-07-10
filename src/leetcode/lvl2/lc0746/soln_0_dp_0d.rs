/// @author: Leon
/// https://leetcode.com/problems/min-cost-climbing-stairs/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/min-cost-climbing-stairs/discuss/476388/4-ways-or-Step-by-step-from-Recursion-greater-top-down-DP-greater-bottom-up-DP-greater-fine-tuning
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_cost_climbing_stairs(costs: Vec<i32>) -> i32 {
        let len_cs: usize = costs.len();
        let mut prev2: i32 = costs[0];
        let mut prev1: i32 = costs[1];
        if len_cs <= 2 {
            return std::cmp::min(prev2, prev1);
        }
        for idx in 2..len_cs {
            let cur = costs[idx] + std::cmp::min(prev2, prev1);
            prev2 = prev1;
            prev1 = cur;
        }
        std::cmp::min(prev2, prev1)
    }
}
