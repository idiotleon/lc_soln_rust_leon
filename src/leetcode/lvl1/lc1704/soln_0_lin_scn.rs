/// @author: Leon
/// https://leetcode.com/problems/determine-if-string-halves-are-alike
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let len_s: usize = s.len();
        let mut cnt1: i32 = 0;
        let mut cnt2: i32 = 0;
        for (idx, ch) in s.chars().enumerate() {
            if idx < len_s / 2 {
                if Self::is_vowel(ch) {
                    cnt1 += 1;
                }
            } else {
                if Self::is_vowel(ch) {
                    cnt2 += 1;
                }
            }
        }
        return cnt1 == cnt2;
    }
    fn is_vowel(ch: char) -> bool {
        let all_vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
        for c in all_vowels {
            if ch == c {
                return true;
            }
        }
        return false;
    }
}
