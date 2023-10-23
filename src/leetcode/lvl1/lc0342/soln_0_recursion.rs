/// @author: Leon
/// https://leetcode.com/problems/power-of-four/
/// Time Complexity:    O(lg(`n`))
/// Space Complexity:   O(lg(`n`))
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        let mut num: i32 = n;
        while num >= 4 {
            if num % 4 != 0 {
                return false;
            }
            num /= 4;
        }
        return num == 1;
    }
}
