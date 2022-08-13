use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/count-common-words-with-one-occurrence/
/// Time Complexity:    O((`len_ws1` + `len_ws2`) * avg_len_wd)
/// Space Complexity:   O((`len_ws1` + `len_ws2`) * avg_len_wd)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let len_ws1: usize = words1.len();
        let len_ws2: usize = words2.len();
        let mut word_to_freq: HashMap<&str, i16> = HashMap::with_capacity(len_ws1 + len_ws2);
        for word in &words1 {
            *word_to_freq.entry(word).or_default() += 1;
        }
        for word in &words2 {
            if let Some(&freq) = word_to_freq.get(&word as &str) {
                if freq > 1 {
                    continue;
                }
                *word_to_freq.entry(word).or_default() -= 1;
            }
        }
        return word_to_freq
            .into_iter()
            .filter(|(_word, freq)| *freq == 0)
            .count() as i32;
    }
}
