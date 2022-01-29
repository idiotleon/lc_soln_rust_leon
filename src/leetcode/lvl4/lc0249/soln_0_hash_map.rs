/// @author: Leon
/// https://leetcode.com/problems/group-shifted-strings/
/// Time Complexity:    O(`_len_strs` * ave_len_str)
/// Space Complexity:   O(`_len_strs` * ave_len_str)
/// Reference:
/// https://leetcode.com/problems/group-shifted-strings/discuss/67442/My-Concise-JAVA-Solution/69322
use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn group_strings(strings: Vec<String>) -> Vec<Vec<String>> {
        let hash_to_strs: HashMap<String, Vec<String>> = {
            let mut map: HashMap<String, Vec<String>> = HashMap::new();
            for string in strings {
                let hash = Self::get_hash(&string.chars().collect());
                map.entry(hash).or_default().push(string);
            }
            map
        };
        hash_to_strs.into_values().collect()
    }
    fn get_hash(chs: &Vec<char>) -> String {
        let len_chs: usize = chs.len();
        const SPLITTER: char = ';';
        let mut ans: String = "".to_owned();
        for idx in 1..len_chs {
            let diff = {
                let mut diff = chs[idx] as i8 - chs[idx - 1] as i8;
                if diff < 0 {
                    diff += 26;
                }
                diff
            };
            ans.push(SPLITTER);
            ans.push_str(&diff.to_string());
        }
        ans
    }
}
