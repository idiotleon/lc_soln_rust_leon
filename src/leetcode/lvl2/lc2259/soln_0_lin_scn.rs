/// @author: Leon
/// https://leetcode.com/problems/remove-digit-from-number-to-maximize-result/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_digit(number: String, digit: char) -> String {
        let len_s: usize = number.len();
        const IMPL: char = '#';
        let mut last: usize = len_s + 1;
        let mut is_removed: bool = false;
        let mut chs: Vec<char> = number.chars().collect();
        for (idx, &ch) in chs.iter().enumerate() {
            if ch == digit {
                if idx + 1 < len_s {
                    if chs[idx + 1] > ch {
                        chs[idx] = IMPL;
                        is_removed = true;
                        break;
                    }
                }
                last = idx;
            }
        }
        if !is_removed {
            chs[last] = IMPL;
        }
        chs.into_iter().filter(|&e| e != IMPL).collect()
    }
}
