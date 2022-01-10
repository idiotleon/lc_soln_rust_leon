/// @author: Leon
/// https://leetcode.com/problems/find-words-that-can-be-formed-by-characters/
/// Time Complexity:    O(`_len_ws` * avg_len_w)
/// Space Complexity:   O(`_len_ws` * avg_len_w)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_characters(words: Vec<String>, chs: String) -> i32 {
        let _len_ws: usize = words.len();
        let freqs_chs: Vec<u8> = {
            let mut res: Vec<u8> = vec![0; 26];
            for ch in chs.chars() {
                res[ch as usize - 'a' as usize] += 1;
            }
            res
        };
        let mut ans: usize = 0;
        for word in words {
            if Self::can_form(&word, &freqs_chs) {
                ans += word.len();
            }
        }
        ans as i32
    }
    fn can_form(word: &str, freqs_chs: &Vec<u8>) -> bool {
        let freqs: Vec<u8> = {
            let mut res: Vec<u8> = vec![0; 26];
            for ch in word.chars() {
                res[ch as usize - 'a' as usize] += 1;
            }
            res
        };
        for (idx, freq) in freqs.into_iter().enumerate() {
            if freq > freqs_chs[idx] {
                return false;
            }
        }
        true
    }
}
