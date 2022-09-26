/// @author: Leon
/// https://leetcode.com/problems/break-a-palindrome/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        let len_s: usize = palindrome.len();
        let mut chs: Vec<char> = palindrome.chars().collect();
        for idx in 0..len_s / 2 {
            if chs[idx] != 'a' {
                chs[idx] = 'a';
                return chs.into_iter().collect();
            }
        }
        if len_s < 2 {
            return "".to_owned();
        }
        chs[len_s - 1] = 'b';
        return chs.into_iter().collect();
    }
}
