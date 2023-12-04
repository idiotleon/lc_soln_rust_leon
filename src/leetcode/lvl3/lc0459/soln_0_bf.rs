/// @author: Leon
/// https://leetcode.com/problems/repeated-substring-pattern/
/// Time Complexity:    O(`len_st`)
/// Space Complexity:   O(`len_st`)
/// Reference:
/// https://leetcode.com/problems/repeated-substring-pattern/editorial/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let t: String = format!("{}{}", s, s);
        let len_st: usize = t.len();
        if t[1..len_st - 1].contains(&s) {
            return true;
        }
        return false;
    }
}
