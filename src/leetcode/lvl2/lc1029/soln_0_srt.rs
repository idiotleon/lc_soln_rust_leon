/// @author: Leon
/// https://leetcode.com/problems/two-city-scheduling/
/// Time Complexity:    O(`len_cts` * lg(`len_cts`))
/// Space Complexity:   O(`len_cts`)
/// Reference:
/// https://leetcode.com/problems/two-city-scheduling/discuss/667786/Java-or-C%2B%2B-or-Python3-or-With-detailed-explanation
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let len_cts: usize = costs.len();
        let mut ans: i32 = 0;
        let refunds: Vec<i32> = {
            let mut refunds: Vec<i32> = vec![0; len_cts];
            for (idx, cost) in costs.into_iter().enumerate() {
                refunds[idx] = cost[1] - cost[0];
                ans += cost[0];
            }
            refunds.sort();
            refunds
        };
        for idx in 0..len_cts / 2 {
            ans += refunds[idx];
        }
        ans
    }
}
