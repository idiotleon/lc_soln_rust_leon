use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/word-break/
/// Time Complexity:    O(`len_s` ^ 2)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let len_s: usize = s.len();
        if s.is_empty() {
            return false;
        }
        let word_set: HashSet<String> = word_dict.into_iter().collect();
        let mut seen: HashSet<usize> = HashSet::with_capacity(len_s);
        return Self::dfs(0, &s, &mut seen, &word_set);
    }
    fn dfs(cur: usize, s: &String, seen: &mut HashSet<usize>, word_set: &HashSet<String>) -> bool {
        let len_s: usize = s.len();
        if cur == len_s {
            return true;
        }
        if seen.contains(&cur) {
            return false;
        }
        for nxt in cur + 1..=len_s {
            let sub = &s[cur..nxt];
            if !word_set.contains(&(sub.to_owned())) {
                continue;
            }
            if Self::dfs(nxt, s, seen, word_set) {
                return true;
            }
        }
        seen.insert(cur);
        return false;
    }
}
