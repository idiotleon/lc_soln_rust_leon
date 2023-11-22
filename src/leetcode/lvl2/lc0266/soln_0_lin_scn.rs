/// @author: Leon
/// https://leetcode.com/problems/palindrome-permutation/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_permute_palindrome(s: String) -> bool {
        let _len_s: usize = s.len();
        let freqs: Vec<u16> = {
            let mut freqs: Vec<u16> = vec![0; 26];
            for ch in s.chars() {
                freqs[ch as usize - 'a' as usize] += 1;
            }
            freqs
        };
        let mut only_once: bool = false;
        for freq in freqs {
            if freq % 2 == 1 {
                if only_once {
                    return false;
                }
                only_once = true;
            }
        }
        return true;
    }
}
