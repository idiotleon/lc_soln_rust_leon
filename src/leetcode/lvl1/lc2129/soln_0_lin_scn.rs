/// @author: Leon
/// https://leetcode.com/problems/capitalize-the-title/
/// Time Complexity:    O(`_len_tl`)
/// Space Complexity:   O(`_len_tl`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn capitalize_title(title: String) -> String {
        let _len_tl: usize = title.len();
        const SPACE: &str = &" ";
        let words: Vec<&str> = title.split(SPACE).collect();
        let len_ws: usize = words.len();
        let mut res: Vec<String> = Vec::with_capacity(len_ws);
        for idx in 0..len_ws {
            res.push(Self::cap_wrd(words[idx]));
        }
        res.join(SPACE)
    }
    fn cap_wrd(word: &str) -> String {
        let len_wd: usize = word.len();
        let mut chs: Vec<char> = word.chars().collect();
        const SPACE: char = ' ';
        if len_wd > 2 {
            for idx in 0..len_wd {
                if idx == 0 || chs[idx - 1] == SPACE {
                    if chs[idx].is_ascii_lowercase() {
                        chs[idx] = chs[idx].to_ascii_uppercase();
                    }
                } else {
                    if chs[idx].is_ascii_uppercase() {
                        chs[idx] = chs[idx].to_ascii_lowercase();
                    }
                }
            }
        } else {
            for idx in 0..len_wd {
                if chs[idx].is_ascii_uppercase() {
                    chs[idx] = chs[idx].to_ascii_lowercase();
                }
            }
        }
        chs.into_iter().collect()
    }
}
