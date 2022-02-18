use std::collections::HashMap;
/// @author: Leon
/// https://leetcode.com/problems/longest-palindrome-by-concatenating-two-letter-words/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/longest-palindrome-by-concatenating-two-letter-words/discuss/1675343/Python3-Java-C%2B%2B-Counting-Mirror-Words-O(n)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        let mut wd_to_freq: HashMap<String, u16> = HashMap::new();
        let mut ans: i32 = 0;
        let mut unpaired: i32 = 0;
        for wd in words {
            let chs: Vec<char> = wd.chars().collect();
            if chs[0] == chs[1] {
                let freq = wd_to_freq.entry(wd).or_default();
                if *freq > 0 {
                    unpaired -= 1;
                    *freq -= 1;
                    ans += 4;
                } else {
                    *freq += 1;
                    unpaired += 1;
                }
            } else {
                let rev: String = chs.into_iter().rev().collect();
                let freq_rev = wd_to_freq.entry(rev).or_default();
                if *freq_rev > 0 {
                    ans += 4;
                    *freq_rev -= 1;
                } else {
                    *wd_to_freq.entry(wd).or_default() += 1;
                }
            }
        }
        if unpaired > 0 {
            ans += 2;
        }
        ans
    }
}
