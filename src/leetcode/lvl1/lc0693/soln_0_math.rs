/// @author: Leon
/// https://leetcode.com/problems/binary-number-with-alternating-bits/
/// Time Complexity:    O(lg(`n`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut num = n;
        let mut prev: i32 = -1;
        while num > 0 {
            let cur = num % 2;
            if cur == prev {
                return false;
            }
            num /= 2;
            prev = cur;
        }
        return true;
    }
}
