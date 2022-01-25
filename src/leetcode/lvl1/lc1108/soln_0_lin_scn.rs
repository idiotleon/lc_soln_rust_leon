/// @author: Leon
/// https://leetcode.com/problems/defanging-an-ip-address/
/// Time Complexity:    O(`_len_adr`)
/// Space Complexity:   O(1) / O(`_len_adr`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        let _len_adr: usize = address.len();
        let mut ans: String = "".to_owned();
        const DOT: char = '.';
        const LEFT_BRACKET: char = '[';
        const RIGHT_BRACKET: char = ']';
        for ch in address.chars() {
            match ch {
                DOT => {
                    ans.push(LEFT_BRACKET);
                    ans.push(DOT);
                    ans.push(RIGHT_BRACKET);
                }
                _ => ans.push(ch),
            }
        }
        ans.to_owned()
    }
}
