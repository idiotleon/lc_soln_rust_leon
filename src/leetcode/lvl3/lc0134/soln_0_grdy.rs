/// @author: Leon
/// https://leetcode.com/problems/gas-station/
/// Time Complexity:    O(`len_gs`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let len_gs: usize = gas.len();
        let mut cur_tank = 0;
        let mut total_tank = 0;
        let mut idx_start: usize = 0;
        for idx in 0..len_gs {
            let balance = gas[idx] - cost[idx];
            cur_tank += balance;
            if cur_tank < 0 {
                idx_start = idx;
                cur_tank = 0;
            }
            total_tank += balance;
        }
        if total_tank >= 0 {
            idx_start as i32
        } else {
            -1
        }
    }
}
