/// @author: Leon
/// https://leetcode.com/problems/truncate-sentence/
///
/// Time Complexity:  O(`_len_s`)
/// Space Complexity: O(1)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn truncate_sentence(str: String, k: i32) -> String {
        let _len_s = str.len();
        let mut cnt: i32 = 0;
        let mut ans = String::new();
        for ch in str.chars() {
            if ch == ' ' {
                cnt += 1;
                if cnt == k {
                    break;
                }
            }
            ans.push(ch);
        }
        ans
    }
}
