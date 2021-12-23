/// https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1) / O(`_len_s`)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let _len_s = s.len();
        let mut ans = String::from("");
        for ch in s.chars() {
            if !ans.is_empty() && ans.chars().last().unwrap() == ch {
                ans.pop();
            } else {
                ans.push(ch);
            }
        }
        ans
    }
}
