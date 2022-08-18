use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/kth-distinct-string-in-an-array/
/// Time Complexity:    O(`len_ss`)
/// Space Complexity:   O(`len_ss`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn kth_distinct(strs: Vec<String>, k: i32) -> String {
        let len_ss: usize = strs.len();
        let str_to_freq: HashMap<&str, u16> = {
            let mut map: HashMap<&str, u16> = HashMap::with_capacity(len_ss);
            for s in &strs {
                *map.entry(s).or_default() += 1;
            }
            map
        };
        let mut kth: i32 = 0;
        for s in &strs {
            if let Some(&freq) = str_to_freq.get(&s as &str) {
                if freq > 1 {
                    continue;
                }
                kth += 1;
                if kth == k {
                    return s.to_owned();
                }
            }
        }
        return "".to_owned();
    }
}
