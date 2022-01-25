/// @author: Leon
/// https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/
/// Time Complexity:    O(`_len_stcs` * `_len_stc`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        let _len_stcs: usize = sentences.len();
        let mut max: u8 = 0;
        for sentence in sentences {
            max = std::cmp::max(max, Self::count_words(&sentence));
        }
        max as i32
    }
    fn count_words(sentence: &str) -> u8 {
        let _len_stc: usize = sentence.len();
        const SPACE: char = ' ';
        let mut cnt: u8 = 0;
        let mut is_space: bool = false;
        for ch in sentence.chars() {
            match ch {
                SPACE => is_space = true,
                _ => {
                    if is_space {
                        cnt += 1;
                    }
                    is_space = false;
                }
            }
        }
        cnt + 1
    }
}
