/// @author: Leon
/// https://leetcode.com/problems/reverse-prefix-of-word/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let len_s: usize = word.len();
        let mut chs: Vec<char> = word.chars().collect();
        for idx in 0..len_s {
            if chs[idx] == ch {
                Self::reverse(idx, &mut chs);
                break;
            }
        }
        return chs.into_iter().collect();
    }
    fn reverse(end: usize, chs: &mut Vec<char>) {
        let mut lo: usize = 0;
        let mut hi: usize = end;
        while lo < hi {
            chs.swap(lo, hi);
            lo += 1;
            hi -= 1;
        }
    }
}
