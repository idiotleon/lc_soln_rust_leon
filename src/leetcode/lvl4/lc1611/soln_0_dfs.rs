/// @author: Leon
/// https://leetcode.com/problems/minimum-one-bit-operations-to-make-integers-zero/
/// Time Complexity:    O(lg(`n`))
/// Space Complexity:   O(lg(`n`))
/// Reference:
/// https://leetcode.com/problems/minimum-one-bit-operations-to-make-integers-zero/editorial/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        let mut k: i32 = 0;
        let mut cur: i32 = 1;
        while cur * 2 <= n {
            cur *= 2;
            k += 1;
        }
        return (1 << (k + 1)) - 1 - Self::minimum_one_bit_operations(n ^ cur);
    }
}
