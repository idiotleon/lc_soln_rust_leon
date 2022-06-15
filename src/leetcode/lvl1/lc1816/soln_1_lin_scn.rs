/// @author: Leon
/// https://leetcode.com/problems/truncate-sentence/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let _len_s: usize = s.len();
        const SPACE: char = ' ';
        let mut ans: String = "".to_owned();
        let word: &mut String = &mut "".to_owned();
        let mut cnt: i32 = 0;
        for ch in s.chars() {
            if ch == SPACE {
                if !word.is_empty() {
                    if !ans.is_empty() {
                        ans.push(SPACE);
                    }
                    ans.push_str(word);
                }
                word.clear();
                cnt += 1;
                if cnt == k {
                    break;
                }
            } else {
                word.push(ch);
            }
        }
        if !word.is_empty() {
            if !ans.is_empty() {
                ans.push(SPACE);
            }
            ans.push_str(word);
        }
        ans
    }
}
