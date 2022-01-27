/// @author: Leon
/// https://leetcode.com/problems/remove-outermost-parentheses/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/remove-outermost-parentheses/discuss/270022/JavaC%2B%2BPython-Count-Opened-Parenthesis
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_outer_parentheses(s: String) -> String {
        let _len_s: usize = s.len();
        const OPEN: char = '(';
        const CLOSED: char = ')';
        let mut stk: u16 = 0;
        let mut ans: String = "".to_owned();
        for ch in s.chars() {
            match ch {
                OPEN => {
                    stk += 1;
                    if stk > 1 {
                        ans.push(ch);
                    }
                }
                CLOSED => {
                    stk -= 1;
                    if stk > 0 {
                        ans.push(ch);
                    }
                }
                _ => unreachable!(),
            }
        }
        ans
    }
}
