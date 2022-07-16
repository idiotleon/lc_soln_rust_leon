/// @author: Leon
/// https://leetcode.com/problems/count-prefixes-of-a-given-string/
/// Time Complexity:    O(`len_s` * avg_len_word)
/// Space Compleixty:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_prefixes(words: Vec<String>, s: String) -> i32 {
        let _len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        let mut cnt: i32 = 0;
        for word in words {
            if Self::is_prefix(word, &chs) {
                cnt += 1;
            }
        }
        cnt
    }
    fn is_prefix(word: String, chs_s: &Vec<char>) -> bool {
        let len_w: usize = word.len();
        let len_s: usize = chs_s.len();
        if len_w > len_s {
            return false;
        }
        let len: usize = std::cmp::min(len_w, len_s);
        let chs_w: Vec<char> = word.chars().collect();
        for idx in 0..len {
            if chs_w[idx] != chs_s[idx] {
                return false;
            }
        }
        true
    }
}
