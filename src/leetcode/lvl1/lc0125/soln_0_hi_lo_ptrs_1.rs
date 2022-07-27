/// @author: Leon
/// https://leetcode.com/problems/valid-palindrome/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let len_s = s.len();
        let chs: Vec<char> = s.chars().collect();
        let mut lo: isize = 0;
        let mut hi: isize = len_s as isize - 1;
        while lo < hi {
            while lo < hi && !chs[lo as usize].is_alphanumeric() {
                lo += 1;
            }
            while lo < hi && !chs[hi as usize].is_alphanumeric() {
                hi -= 1;
            }
            if chs[lo as usize].to_ascii_lowercase() != chs[hi as usize].to_ascii_lowercase() {
                return false;
            }
            lo += 1;
            hi -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let s = "a.".to_owned();
        assert_eq!(true, Solution::is_palindrome(s));
    }
}
