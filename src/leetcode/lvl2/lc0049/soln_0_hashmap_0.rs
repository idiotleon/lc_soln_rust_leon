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
        let mut hash_to_words: HashMap<String, Vec<String>> = HashMap::new();

        for str in strs {
            let freqs: Vec<u32> = {
                let mut tmp: Vec<u32> = vec![0; 26];
                for ch in str.chars() {
                    let idx = ch as u8 - 'a' as u8;
                    tmp[idx as usize] += 1;
                }

                tmp
            };

            let hash = format!("{:?}", freqs);
            hash_to_words.entry(hash).or_insert(vec![]).push(str);
        }

        hash_to_words.values().cloned().collect()
    }
}
