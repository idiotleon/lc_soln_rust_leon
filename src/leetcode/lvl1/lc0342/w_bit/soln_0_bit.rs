/// @author: Leon
/// https://leetcode.com/problems/power-of-four/
/// Time Complexity:    O(lg(`n`))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/power-of-four/solutions/4197440/100-power-of-two-check-masking/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        if n > 0 && (n & (n - 1) == 0) {
            let mut cnt_zero = 0;
            let mut num = n;
            while num > 1 {
                cnt_zero += 1;
                num >>= 1;
            }
            return cnt_zero % 2 == 0;
        }
        return false;
    }
}
