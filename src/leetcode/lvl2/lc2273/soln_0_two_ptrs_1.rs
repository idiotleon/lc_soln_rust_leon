/// @author: Leon
/// https://leetcode.com/problems/find-resultant-array-after-removing-anagrams/
/// Time Complexity:    O(`_len_ws` * avg_len_word)
/// Space Complexity:   O(`_len_ws` * avg_len_word)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let len_ws: usize = words.len();
        let mut ans: Vec<String> = Vec::with_capacity(len_ws);
        ans.push(words[0].to_owned());
        let mut idx: usize = 1;
        while idx < len_ws {
            while idx < len_ws && Self::is_anagram(&ans.last().unwrap(), &words[idx]) {
                idx += 1;
            }
            if idx < len_ws {
                ans.push(words[idx].to_owned());
            }
            idx += 1;
        }
        ans
    }
    fn is_anagram(word1: &str, word2: &str) -> bool {
        let freqs1: Vec<u8> = Self::get_hash(word1);
        let freqs2: Vec<u8> = Self::get_hash(word2);
        for (idx, &freq1) in freqs1.iter().enumerate() {
            if freqs2[idx] != freq1 {
                return false;
            }
        }
        true
    }
    fn get_hash(word: &str) -> Vec<u8> {
        let mut freqs: Vec<u8> = vec![0; 26];
        for ch in word.chars() {
            freqs[ch as usize - 'a' as usize] += 1;
        }
        freqs
    }
}
