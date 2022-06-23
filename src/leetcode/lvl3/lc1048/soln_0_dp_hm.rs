/// @author: Leon
/// https://leetcode.com/problems/longest-string-chain/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/longest-string-chain/discuss/887186/Rust-translated-60ms-100
/// https://leetcode.com/problems/longest-string-chain/discuss/294890/JavaC%2B%2BPython-DP-Solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        use std::collections::HashMap;
        let mut words = words;
        words.sort_by_key(|s| s.len());
        let mut dp = HashMap::<String, i32>::new();
        let mut longest: i32 = 0;
        for w in words.iter() {
            let mut best = 0;
            for i in 0..w.len() {
                let mut predecessor = String::new();
                predecessor.push_str(&w[..i]);
                predecessor.push_str(&w[i + 1..]);
                dp.entry(predecessor.clone()).or_default();
                let len = dp[&predecessor];
                if 1 + len > best {
                    best = 1 + len;
                }
            }
            dp.insert(w.to_owned(), best);
            if best > longest {
                longest = best;
            }
        }
        longest
    }
}
