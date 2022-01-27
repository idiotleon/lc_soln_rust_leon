use std::collections::HashSet;
/// @author: Leon
/// https://leetcode.com/problems/unique-morse-code-words/
/// Time Complexity:    O(`_len_ws` * ave_len_wrd)
/// Space Complexity:   O(`_len_ws` * ave_len_wrd)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let _len_ws: usize = words.len();
        const MORSE_CODES: &[&str] = &[
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
            "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
            "-.--", "--..",
        ];
        let mut seen: HashSet<String> = HashSet::new();
        for word in words {
            let mut transformed: String = "".to_owned();
            for ch in word.chars() {
                transformed.push_str(MORSE_CODES[ch as usize - 'a' as usize]);
            }
            seen.insert(transformed);
        }
        seen.len() as i32
    }
}
