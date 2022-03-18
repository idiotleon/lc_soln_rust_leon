use std::collections::VecDeque;
/// @author: Leon
/// https://leetcode.com/problems/remove-duplicate-letters/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(`_len_s`)
/// Reference:
/// https://leetcode.com/problems/remove-duplicate-letters/discuss/889778/Rust-stk-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let _len_s = s.len();
        let chs: Vec<char> = s.chars().collect();
        let mut freqs: [u16; 26] = {
            let mut freqs = [0; 26];
            for &ch in &chs {
                freqs[ch as usize - 'a' as usize] += 1;
            }
            freqs
        };
        let mut stk: VecDeque<char> = VecDeque::with_capacity(26);
        let mut exists: [bool; 26] = [false; 26];
        for &ch in &chs {
            let idx = ch as usize - 'a' as usize;
            freqs[idx] -= 1;
            if exists[idx] {
                continue;
            }
            while let Some(&top) = stk.back() {
                let idx_top = top as usize - 'a' as usize;
                if ch < top && freqs[idx_top] > 0 {
                    exists[idx_top] = false;
                    stk.pop_back();
                } else {
                    break;
                }
            }
            stk.push_back(ch);
            exists[idx] = true;
        }
        stk.iter().collect::<String>()
    }
}
