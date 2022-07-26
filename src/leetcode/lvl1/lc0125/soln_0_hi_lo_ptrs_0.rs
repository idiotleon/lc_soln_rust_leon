/// https://leetcode.com/problems/valid-palindrome/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(`_len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let _len_s = s.len();
        let chs: Vec<char> = s.chars().filter(|&a| a.is_alphanumeric()).collect();
        let len_c = chs.len();
        let mut lo: usize = 0;
        let mut hi: usize = len_c - 1;
        while lo < hi {
            if chs[lo].to_ascii_lowercase() != chs[hi].to_ascii_lowercase() {
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
