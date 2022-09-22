/// @author: Leon
/// https://leetcode.com/problems/reverse-words-in-a-string-iii/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_words(s: String) -> String {
        const SPACE: char = ' ';
        let len_s: usize = s.len();
        let mut chs: Vec<char> = s.chars().collect();
        let mut lo: usize = 0;
        let mut hi: usize = 0;
        while hi < len_s {
            while hi < len_s && chs[hi] != SPACE {
                hi += 1;
            }
            Self::reverse(lo, hi - 1, &mut chs);
            hi += 1;
            lo = hi;
        }
        return chs.into_iter().collect();
    }
    fn reverse(mut lo: usize, mut hi: usize, chs: &mut Vec<char>) {
        while lo < hi {
            chs.swap(lo, hi);
            lo += 1;
            hi -= 1;
        }
    }
}
