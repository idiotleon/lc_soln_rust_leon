/// https://leetcode.com/problems/longest-string-chain/
///
/// Time Complexity:    O()
/// Space Complexity:   O()
///
/// Reference:
/// https://leetcode.com/problems/longest-string-chain/discuss/1213689/Rust-DP-solution
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        use std::collections::HashMap;
        let mut words = words;
        words.sort_by_key(|s| s.len());
        let mut dp = HashMap::<&String, i32>::new();
        let mut longest: i32 = 0;

        for w in &words {
            let best = (0..w.len())
                .filter_map(|i| dp.get(&(String::new() + &w[0..i] + &w[i + 1..])))
                .max()
                .unwrap_or(&0)
                + 1;
            dp.insert(w, best);
            longest = longest.max(best);
        }
        longest
    }
}
