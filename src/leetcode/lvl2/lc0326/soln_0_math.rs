/// @author: Leon
/// https://leetcode.com/problems/power-of-three/
/// Time Complexity:    O(1)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        let mut n = n;
        if n > 1 {
            while n % 3 == 0 {
                n /= 3;
            }
        }
        return n == 1;
    }
}
