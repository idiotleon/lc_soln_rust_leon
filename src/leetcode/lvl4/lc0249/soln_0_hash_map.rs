/// https://leetcode.com/problems/group-shifted-strings/
/// Time Complexity:    O(`_len_strs` * ave_len_str)
/// Space Complexity:   O(`_len_strs` * ave_len_str)
use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    const SPLITTER: char = ';';
    pub fn group_strings(strings: Vec<String>) -> Vec<Vec<String>> {
        let _len_strs = strings.len();
        let hash_to_strs: HashMap<String, Vec<String>> = {
            let mut tmp: HashMap<String, Vec<String>> = HashMap::new();
            for s in strings {
                let hash = Self::get_hash(&s.chars().collect());
                tmp.entry(hash).or_insert(vec![]).push(s);
            }
            tmp
        };
        hash_to_strs
            .into_iter()
            .map(|(_key, value)| value)
            .collect()
    }
    fn get_hash(chs: &Vec<char>) -> String {
        let len_s = chs.len();
        let mut res: String = String::new();
        for idx in 1..len_s {
            let diff = {
                let mut tmp = chs[idx] as i8 - chs[idx - 1] as i8;
                if tmp < 0 {
                    tmp += 26;
                }
                tmp
            };
            res.push(Self::SPLITTER);
            res.push_str(&diff.to_string());
        }
        res
    }
}
