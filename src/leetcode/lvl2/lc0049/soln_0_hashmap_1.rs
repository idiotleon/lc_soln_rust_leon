/// @author: Leon
/// https://leetcode.com/problems/group-anagrams/
///
/// Time Complexity:    O(`n_words` * ave_len_word)
/// Space Complexity:   O(`n_words` * ave_len_word)
use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut bitmask_to_words: HashMap<String, Vec<String>> = HashMap::new();

        for str in strs {
            let chs: Vec<char> = {
                let mut chs: Vec<char> = str.chars().collect();
                chs.sort();

                chs
            };

            let hash = format!("{:?}", chs);
            bitmask_to_words.entry(hash).or_insert(vec![]).push(str);
        }

        bitmask_to_words.values().cloned().collect()
    }
}
