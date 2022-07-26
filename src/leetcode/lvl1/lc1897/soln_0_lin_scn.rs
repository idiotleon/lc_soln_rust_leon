/// @author: Leon
/// https://leetcode.com/problems/redistribute-characters-to-make-all-strings-equal/
/// Time Complexity:    O(`len_ws` * ave_len_wsord)
/// Space Complexity:   O(26) ~ O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let len_ws = words.len();
        let freqs = {
            let mut tmp = vec![0; 26];
            for word in words {
                for ch in word.chars() {
                    tmp[ch as usize - 'a' as usize] += 1;
                }
            }
            tmp
        };
        for freq in freqs {
            if freq % len_ws != 0 {
                return false;
            }
        }
        true
    }
}
