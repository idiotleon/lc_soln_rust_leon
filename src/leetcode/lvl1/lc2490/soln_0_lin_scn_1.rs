/// @author: Leon
/// https://leetcode.com/problems/circular-sentence/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let len_s: usize = sentence.len();
        const SPACE: char = ' ';
        let chs: Vec<char> = sentence.chars().collect();
        for (idx, &ch) in chs.iter().enumerate() {
            if ch == SPACE {
                if idx > 0 && idx < len_s - 1 {
                    if chs[idx - 1] != chs[idx + 1] {
                        return false;
                    }
                }
            }
        }
        return chs[0] == chs[len_s - 1];
    }
}
