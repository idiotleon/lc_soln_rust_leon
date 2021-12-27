/// https://leetcode.com/problems/a-number-after-a-double-reversal/
/// Time Complexity:    O(1)
/// Space Complexity:   O(1)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_same_after_reversals(num: i32) -> bool {
        num == 0 || num % 10 != 0
    }
}
