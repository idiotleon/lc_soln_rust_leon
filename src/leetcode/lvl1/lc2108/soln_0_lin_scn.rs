/// https://leetcode.com/problems/find-first-palindromic-string-in-the-array/
/// Time Complexity:    O(`_len_w` * `len_w`)
/// Space Complexity:   O(`len_w`)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn first_palindrome(words: Vec<String>) -> String {
        let _len_w = words.len();
        for word in words {
            if Self::is_palindrome(&word) {
                return word;
            };
        }
        "".to_owned()
    }
    fn is_palindrome(word: &str) -> bool {
        let len_w = word.len();
        let chs: Vec<char> = word.chars().collect();
        let mut lo: usize = 0;
        let mut hi: usize = len_w - 1;
        while lo < hi {
            if chs[lo] != chs[hi] {
                return false;
            }
            lo += 1;
            hi -= 1;
        }
        true
    }
}
