// NOT correct
// index out of bound

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut memo = vec![vec![0; n]; m];
        return Self::dfs(m - 1, n - 1, &mut memo);
    }

    fn dfs(r: usize, c: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
        if r < 0 || c < 0 {
            return 0;
        }
        if r == 0 && c == 0 {
            return 1;
        }
        if memo[r][c] > 0 {
            return memo[r][c];
        }
        let cnt = Self::dfs(r - 1, c, memo) + Self::dfs(r, c - 1, memo);
        memo[r][c] = cnt;
        return cnt;
    }
}
