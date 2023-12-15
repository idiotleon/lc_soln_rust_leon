use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/destination-city/
/// Time Complexity:    O(`len_ps` * avg_len_path)
/// Space Complexity:   O(`len_ps` * avg_len_path)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let len_ps: usize = paths.len();
        let (dest, outdegree): (Vec<&str>, HashMap<&str, u8>) = {
            let mut dest: Vec<&str> = Vec::with_capacity(len_ps);
            let mut outdegree: HashMap<&str, u8> = HashMap::with_capacity(len_ps);
            for path in &paths {
                let from = &path[0][..];
                let to = &path[1][..];
                *outdegree.entry(&from).or_default() += 1;
                dest.push(&to);
            }
            (dest, outdegree)
        };
        for path in dest {
            if *outdegree.get(&path).unwrap_or(&0) == 0 {
                return path.to_owned();
            }
        }
        unreachable!()
    }
}
