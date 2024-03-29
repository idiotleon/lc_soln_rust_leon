/// @author: Leon
/// https://leetcode.com/problems/maximum-xor-product/
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/maximum-xor-product/solutions/4304616/keep-greedy-keep-stupid/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_xor_product(a: i64, b: i64, n: i32) -> i32 {
        const MOD: u128 = 1e9 as u128 + 7;
        let mut a: u128 = a as u128;
        let mut b: u128 = b as u128;
        for idx in (0..n).rev() {
            let bit_mask = 1 << idx;
            if a & bit_mask == b & bit_mask {
                a |= bit_mask;
                b |= bit_mask;
            } else if a * b < (a ^ bit_mask) * (b ^ bit_mask) {
                a ^= bit_mask;
                b ^= bit_mask;
            }
        }
        return ((a * b) % MOD) as i32;
    }
}
