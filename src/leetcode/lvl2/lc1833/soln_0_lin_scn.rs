/// @author: Leon
/// https://leetcode.com/problems/maximum-ice-cream-bars/
/// Time Complexity:    (`len_cs` * lg(`len_cs`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
        let len_cs: usize = costs.len();
        let sorted: Vec<i32> = {
            let mut sorted = costs;
            sorted.sort();
            sorted
        };
        let mut sum: i32 = 0;
        for (idx, cost) in sorted.into_iter().enumerate() {
            sum += cost;
            if sum > coins {
                // to turn 0-indexed into 1-indexed
                return idx as i32 - 1 + 1;
            }
        }
        return len_cs as i32;
    }
}
