/// https://leetcode.com/problems/power-of-two/
/// Time Complexity:    O(1)
/// Space Complexity:   O(1)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        n & (n - 1) == 0
    }
}
