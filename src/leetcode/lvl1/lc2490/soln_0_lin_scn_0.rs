/// @author: Leon
/// https://leetcode.com/problems/circular-sentence/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let len_s: usize = sentence.len();
        if len_s == 1 {
            return true;
        }
        const SPACE: char = ' ';
        const IMPL: char = '#';
        let mut first: char = IMPL;
        let mut last: char = IMPL;
        let mut prev: char = IMPL;
        let mut prev_two: char = IMPL;
        for (idx, ch) in sentence.chars().into_iter().enumerate() {
            if idx == 0 {
                first = ch;
            } else if idx == len_s - 1 {
                last = ch;
            }
            if prev == SPACE && prev_two != ch {
                return false;
            }
            prev_two = prev;
            prev = ch;
        }
        return first == last;
    }
}
