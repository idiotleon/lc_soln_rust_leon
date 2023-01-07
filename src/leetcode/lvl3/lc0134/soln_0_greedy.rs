/// @author: Leon
/// https://leetcode.com/problems/gas-station/
/// Time Complexity:    O(`len_gs`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, costs: Vec<i32>) -> i32 {
        let len_gs: usize = gas.len();
        let mut tank_cur = 0;
        let mut tank_total = 0;
        let mut idx_start: usize = 0;
        for idx in 0..len_gs {
            let balance = gas[idx] - costs[idx];
            tank_cur += balance;
            if tank_cur < 0 {
                idx_start = 1 + idx;
                tank_cur = 0;
            }
            tank_total += balance;
        }
        return if tank_total >= 0 {
            idx_start as i32
        } else {
            -1
        };
    }
}
