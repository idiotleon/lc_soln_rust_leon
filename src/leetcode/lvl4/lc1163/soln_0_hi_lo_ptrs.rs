/// @author: Leon
/// https://leetcode.com/problems/last-substring-in-lexicographical-order/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
/// Reference:
/// https://leetcode.com/problems/last-substring-in-lexicographical-order/discuss/362387/JavaPython-3-Two-short-O(n)-codes-language%3A-2-pointers-and-encoding.
/// https://cp-algorithms.com/string/lyndon_factorization.html
/// https://en.wikipedia.org/wiki/Lexicographically_minimal_string_rotation
/// https://leetcode.com/problems/last-substring-in-lexicographical-order/discuss/362387/JavaPython-3-Two-short-O(n)-codes-language:-2-pointers-and-encoding./336042
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn last_substring(s: String) -> String {
        let len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        // the offset
        let mut len: usize = 0;
        let mut lo: usize = 0;
        let mut hi: usize = 1;
        while lo + len < len_s && hi + len < len_s {
            let ch_hi: char = chs[hi + len];
            let ch_lo: char = chs[lo + len];
            if ch_lo == ch_hi {
                len += 1;
            } else {
                if ch_lo < ch_hi {
                    lo += len + 1;
                } else {
                    hi += len + 1;
                }
                if lo == hi {
                    hi += 1;
                }
                len = 0;
            }
        }
        (&s[lo..]).to_owned()
    }
}
