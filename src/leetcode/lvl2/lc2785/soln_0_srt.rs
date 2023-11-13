/// @author: Leon
/// https://leetcode.com/problems/sort-vowels-in-a-string/
/// Time Complexity:    O(`len_s` * lg(`len_s`))
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let len_s: usize = s.len();
        let mut chs: Vec<char> = s.chars().collect();
        let sorted: Vec<char> = {
            let mut res: Vec<char> = Vec::with_capacity(len_s);
            for &ch in &chs {
                if Self::is_vowel(ch) {
                    res.push(ch);
                }
            }
            res.sort();
            res
        };
        let mut idx_s: usize = 0;
        for idx in 0..len_s {
            if Self::is_vowel(chs[idx]) {
                chs[idx] = sorted[idx_s];
                idx_s += 1;
            }
        }
        return chs.into_iter().collect();
    }
    fn is_vowel(ch: char) -> bool {
        let all_vowels: Vec<char> = vec!['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u'];
        for &c in &all_vowels {
            if c == ch {
                return true;
            }
        }
        return false;
    }
}
