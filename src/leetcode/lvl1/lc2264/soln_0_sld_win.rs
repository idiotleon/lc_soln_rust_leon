/// @author: Leon
/// https://leetcode.com/problems/largest-3-same-digit-number-in-string/
/// Time Complexity:    O()
/// Space Complexity:   O()
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let len_s: usize = num.len();
        const LEN: usize = 3;
        const IMPL: char = '#';
        let chs: Vec<char> = num.chars().collect();
        let mut largest: char = IMPL;
        let mut idx: usize = 2;
        while idx < len_s {
            if chs[idx] == chs[idx - 1] && chs[idx - 1] == chs[idx - 2] {
                if chs[idx] == IMPL || chs[idx] > largest {
                    largest = chs[idx];
                }
            }
            idx += 1;
        }
        if largest != IMPL {
            largest.to_string().repeat(LEN)
        } else {
            "".to_owned()
        }
    }
}
