use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/concatenated-words/
/// Time Complexity:    O()
/// Space Complexity:   O()
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
        let len_ws: usize = words.len();
        let word_set: HashSet<&str> = words.iter().map(|w| &w[..]).collect();
        let mut failure: HashSet<String> = HashSet::with_capacity(len_ws);
        let mut ans: Vec<String> = Vec::with_capacity(len_ws);
        for idx in 0..len_ws {
            let word = &words[idx][..];
            if Self::dfs(word, &word_set, &mut failure) {
                ans.push(word.to_owned());
            }
        }
        return ans;
    }
    fn dfs(word: &str, word_set: &HashSet<&str>, failure: &mut HashSet<String>) -> bool {
        if failure.contains(&word.to_owned()) {
            return false;
        }
        let len_w: usize = word.len();
        for idx in 1..len_w {
            if word_set.contains(&word[0..idx]) {
                let suffix = &word[idx..];
                if word_set.contains(suffix) || Self::dfs(suffix, word_set, failure) {
                    return true;
                }
            }
        }
        failure.insert(word.to_owned());
        return false;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let words: Vec<String> = vec![
            "cat".to_owned(),
            "cats".to_owned(),
            "catsdogcats".to_owned(),
            "dog".to_owned(),
            "dogcatsdog".to_owned(),
            "hippopotamuses".to_owned(),
            "rat".to_owned(),
            "ratcatdogcat".to_owned(),
        ];
        let actual: Vec<String> = Solution::find_all_concatenated_words_in_a_dict(words);
        let expected: Vec<String> = vec![
            "catsdogcats".to_owned(),
            "dogcatsdog".to_owned(),
            "ratcatdogcat".to_owned(),
        ];
        assert_eq!(expected, actual);
    }
}
