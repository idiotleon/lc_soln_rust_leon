/// @author: Leon
/// https://leetcode.com/problems/generate-parentheses/
/// Time Complexity:    O(2 ^ `n`)
/// Space Complexity:   O(`n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    const PAREN_OPEN: char = '(';
    const PAREN_CLOSED: char = ')';
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut ans: Vec<String> = Vec::new();
        Self::backtrack(0, 0, &mut "".to_owned(), n as usize, &mut ans);
        ans
    }
    fn backtrack(
        cnt_open: usize,
        cnt_closed: usize,
        cur: &mut String,
        n: usize,
        res: &mut Vec<String>,
    ) {
        if cur.len() == n * 2 {
            res.push(cur.to_owned());
            return;
        }
        if cnt_open < n {
            cur.push(Self::PAREN_OPEN);
            Self::backtrack(cnt_open + 1, cnt_closed, cur, n, res);
            cur.pop();
        }
        if cnt_closed < cnt_open {
            cur.push(Self::PAREN_CLOSED);
            Self::backtrack(cnt_open, cnt_closed + 1, cur, n, res);
            cur.pop();
        }
    }
}
