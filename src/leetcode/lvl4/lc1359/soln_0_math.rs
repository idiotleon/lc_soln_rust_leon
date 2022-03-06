/// @author: Leon
/// https://leetcode.com/problems/count-all-valid-pickup-and-delivery-options/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/count-all-valid-pickup-and-delivery-options/discuss/516968/JavaC%2B%2BPython-Easy-and-Concise
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        const MOD: i64 = 1e9 as i64 + 7;
        let n: i64 = n as i64;
        let mut cnt: i64 = 1;
        for num in 1..=n {
            cnt = cnt * (num * 2 - 1) * num % MOD;
        }
        cnt as i32
    }
}
