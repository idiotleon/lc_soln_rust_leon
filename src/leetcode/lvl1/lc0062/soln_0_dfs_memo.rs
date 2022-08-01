/// @author: Leon
/// https://leetcode.com/problems/unique-paths/
/// Time Complexity:    O(`m` * `n`)
/// Space Complexity:   O(`m` * `n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut memo: Vec<Vec<Option<i32>>> = vec![vec![None; n]; m];
        return Self::dfs(m - 1, n - 1, &mut memo);
    }
    fn dfs(r: usize, c: usize, memo: &mut Vec<Vec<Option<i32>>>) -> i32 {
        if r == 0 && c == 0 {
            return 1;
        }
        if let Some(m) = memo[r][c] {
            return m;
        }
        let mut cnt = 0;
        if r > 0 {
            cnt += Self::dfs(r - 1, c, memo)
        }
        if c > 0 {
            cnt += Self::dfs(r, c - 1, memo);
        }
        memo[r][c] = Some(cnt);
        return cnt;
    }
}
