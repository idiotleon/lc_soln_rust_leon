/// @author: Leon
/// https://leetcode.com/problems/removing-stars-from-a-string/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_stars(s: String) -> String {
        let len_s: usize = s.len();
        const ASTERISK: char = '*';
        let mut chs: Vec<char> = s.chars().collect();
        let mut lo: usize = 0;
        let mut hi: usize = 0;
        while hi < len_s {
            if chs[hi] == ASTERISK {
                if lo > 0 {
                    lo -= 1;
                }
            } else {
                chs[lo] = chs[hi];
                lo += 1;
            }
            hi += 1;
        }
        return chs[..lo].into_iter().collect::<String>();
    }
}
