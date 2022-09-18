/// @author: Leon
/// https://leetcode.com/problems/smallest-even-multiple/
/// Time Complexity:    O(1)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        return if n % 2 == 0 { n } else { n * 2 };
    }
}
