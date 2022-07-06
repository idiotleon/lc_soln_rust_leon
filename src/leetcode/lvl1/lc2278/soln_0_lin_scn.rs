/// @author: Leon
/// https://leetcode.com/problems/percentage-of-letter-in-string/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn percentage_letter(s: String, letter: char) -> i32 {
        let len_s: usize = s.len();
        let cnt: usize = s.chars().filter(|&c| c == letter).count();
        (cnt as i32 * 100) / len_s as i32
    }
}
