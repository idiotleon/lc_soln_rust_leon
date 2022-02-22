/// @author: Leon
/// https://leetcode.com/problems/generate-parentheses/
/// Time Complexity:    O(2 ^ `n`)
/// Space Complexity:   O(`n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut paths: Vec<String> = Vec::new();
        let mut path: String = "".to_owned();
        Self::backtrack(0, 0, n, &mut path, &mut paths);
        paths
    }
    fn backtrack(
        cnt_open: i32,
        cnt_closed: i32,
        n: i32,
        path: &mut String,
        paths: &mut Vec<String>,
    ) {
        if path.len() as i32 == n * 2 {
            paths.push(path.to_owned());
            return;
        }
        const OPEN: char = '(';
        const CLOSED: char = ')';
        if cnt_open < n {
            path.push(OPEN);
            Self::backtrack(cnt_open + 1, cnt_closed, n, path, paths);
            path.pop();
        }
        if cnt_closed < cnt_open {
            path.push(CLOSED);
            Self::backtrack(cnt_open, cnt_closed + 1, n, path, paths);
            path.pop();
        }
    }
}
