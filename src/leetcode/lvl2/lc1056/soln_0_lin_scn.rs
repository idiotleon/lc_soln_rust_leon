/// @author: Leon
/// https://leetcode.com/problems/confusing-number/
/// Time Complexity:    O(1)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    const MAP: &'static [i32] = &[0, 1, -1, -1, -1, -1, 9, -1, 8, 6];
    pub fn confusing_number(n: i32) -> bool {
        let mut src = n;
        let mut reversed = 0;
        while src > 0 {
            let digit = src % 10;
            src /= 10;
            if Self::MAP[digit as usize] != -1 {
                reversed *= 10;
                reversed += Self::MAP[digit as usize];
            } else {
                return false;
            }
        }
        return reversed != n;
    }
}
