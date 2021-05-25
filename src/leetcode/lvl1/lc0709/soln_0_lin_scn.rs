/// https://leetcode.com/problems/to-lower-case/
/// 
/// Time Complexity:    O(L)
/// Space Complexity:   O(1)?
/// 
/// Reference:
/// https://leetcode.com/problems/to-lower-case/discuss/502373/Rust-Learn-Rust-Everyday
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn to_lower_case(s: String) -> String {
        s.chars()
            .map(|ch| {
                if ch >= 'A' && ch <= 'Z' {
                    ((ch as u8) - ('A' as u8) + ('a' as u8)) as char
                } else {
                    ch
                }
            })
            .collect::<String>()
    }
}
