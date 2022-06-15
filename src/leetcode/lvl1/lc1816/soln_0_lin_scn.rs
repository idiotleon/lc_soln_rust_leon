/// @author: Leon
/// https://leetcode.com/problems/truncate-sentence/
/// Time Complexity:  O(`_len_s`)
/// Space Complexity: O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn truncate_sentence(s: String, k: i32) -> String {
        let _len_s: usize = s.len();
        const SPACE: char = ' ';
        let ans: &mut String = &mut "".to_owned();
        let mut cnt: i32 = 0;
        for ch in s.chars() {
            if ch == SPACE {
                cnt += 1;
                if cnt == k {
                    break;
                }
            }
            ans.push(ch);
        }
        ans.to_owned()
    }
}
