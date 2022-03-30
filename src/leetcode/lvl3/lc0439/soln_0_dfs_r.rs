/// @author: Leon
/// https://leetcode.com/problems/ternary-expression-parser/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
/// Reference:
/// https://leetcode.com/problems/ternary-expression-parser/discuss/92166/Very-easy-1-pass-Stack-Solution-in-JAVA-(NO-STRING-CONCAT)/96758
/// https://leetcode.com/problems/ternary-expression-parser/discuss/92166/Very-easy-1-pass-Stack-Solution-in-JAVA-(NO-STRING-CONCAT)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn parse_ternary(expression: String) -> String {
        let mut idx = 0;
        let chs: Vec<char> = expression.chars().collect();
        Self::dfs(&mut idx, &chs).to_string()
    }
    fn dfs(idx: &mut usize, chs: &Vec<char>) -> char {
        let len_s: usize = chs.len();
        let ch = chs[*idx];
        if *idx + 1 == len_s || chs[*idx + 1] == ':' {
            *idx += 2;
            return ch;
        }
        *idx += 2;
        let ch_first = Self::dfs(idx, chs);
        let ch_sec = Self::dfs(idx, chs);
        if ch == 'T' {
            ch_first
        } else {
            ch_sec
        }
    }
}
