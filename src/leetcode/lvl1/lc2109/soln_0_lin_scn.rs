/// https://leetcode.com/problems/adding-spaces-to-a-string/
/// Time Complexity:    O(`_len_str`)
/// Space Complexity:   O(1)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<usize>) -> String {
        let mut ans: String = "".to_owned();
        let _len_str: usize = s.len();
        let len_sp: usize = spaces.len();
        let mut idx_s: usize = 0;
        for (idx_ch, ch) in s.chars().enumerate() {
            ans.push(ch);
            if idx_s < len_sp && idx_ch == spaces[idx_s] {
                ans.push_str(" ");
                idx_s += 1;
            }
        }
        ans.to_string()
    }
}
