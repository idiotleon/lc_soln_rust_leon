use std::collections::{HashMap, HashSet};

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
        let ws1: HashSet<&str> = {
            let mut map: HashMap<&str, u16> = HashMap::with_capacity(len_ws1);
            for word in &words1 {
                *map.entry(word).or_default() += 1;
            }
            map.into_iter()
                .filter(|(_word, freq)| *freq == 1)
                .map(|(word, _freq)| word)
                .collect()
        };
        let ws2: HashSet<&str> = {
            let mut map: HashMap<&str, u16> = HashMap::with_capacity(len_ws2);
            for word in &words2 {
                *map.entry(word).or_default() += 1;
            }
            map.into_iter()
                .filter(|(_word, freq)| *freq == 1)
                .map(|(word, _freq)| word)
                .collect()
        };
        let mut cnt: i32 = 0;
        for word in ws1.iter() {
            if ws2.contains(&word as &str) {
                cnt += 1;
            }
        }
        return cnt;
    }
}
