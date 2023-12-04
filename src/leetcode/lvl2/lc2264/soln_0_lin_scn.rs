/// @author: Leon
/// https://leetcode.com/problems/largest-3-same-digit-number-in-string/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_good_integer(s: String) -> String {
        const IMPS: char = '#';
        let len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        let mut len: u8 = 1;
        let mut idx: usize = 1;
        let mut ans: char = IMPS;
        while idx < len_s {
            if chs[idx - 1] == chs[idx] {
                len += 1;
                if len == 3 {
                    if ans == IMPS || ans < chs[idx] {
                        ans = chs[idx];
                    }
                }
            } else {
                len = 1;
            }
            idx += 1;
        }
        return if ans == IMPS {
            "".to_owned()
        } else {
            format!("{}{}{}", ans, ans, ans)
        };
    }
}
