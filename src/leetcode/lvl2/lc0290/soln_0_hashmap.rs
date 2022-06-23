use std::collections::{HashMap, HashSet};

/// @author: Leon
/// https://leetcode.com/problems/word-pattern/
/// Time Complexity:    O(`len_ws` * avg_len_word)
/// Space Complexity:   O(`len_ws` * avg_len_word)
/// Reference:
/// https://leetcode.com/problems/word-pattern/discuss/73399/Very-fast-(3ms)-Java-Solution-using-HashMap
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let len_p: usize = pattern.len();
        let chs_p: Vec<char> = pattern.chars().collect();
        const SPACE: char = ' ';
        let words: Vec<&str> = s.split(SPACE).collect();
        let len_ws: usize = words.len();
        if len_p != len_ws {
            return false;
        }
        let mut ch_to_str: HashMap<char, String> = HashMap::new();
        let mut seen: HashSet<String> = HashSet::new();
        for (idx, ch) in chs_p.into_iter().enumerate() {
            let word = words[idx];
            match ch_to_str.get(&ch) {
                Some(wd) => {
                    if wd != word {
                        return false;
                    }
                }
                None => {
                    if !seen.insert(word.to_owned()) {
                        return false;
                    }
                    ch_to_str.insert(ch, word.to_owned());
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let pattern = "abba".to_owned();
        let s = "dog cat cat dog".to_owned();
        let actual = Solution::word_pattern(pattern, s);
        let expected = true;
        assert_eq!(expected, actual);
    }
}
