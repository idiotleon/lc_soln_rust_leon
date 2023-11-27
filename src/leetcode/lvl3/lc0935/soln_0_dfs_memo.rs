/// @author: Leon
/// https://leetcode.com/problems/knight-dialer/
/// Time Complexity:    O()
/// Space Complexity:   O()
struct Solution;

#[allow(dead_code)]
impl Solution {
    const MOD: i32 = 1e9 as i32 + 7;
    const JUMPS: &'static [&'static [i32]] = &[
        &[4, 6],
        &[6, 8],
        &[7, 9],
        &[4, 8],
        &[3, 9, 0],
        &[],
        &[1, 7, 0],
        &[2, 6],
        &[1, 3],
        &[2, 4],
    ];
    pub fn knight_dialer(n: i32) -> i32 {
        let n: usize = n as usize;
        let mut memo: Vec<Vec<Option<i32>>> = vec![vec![None; 10]; n + 1];
        let mut ans: i32 = 0;
        for square in 0..10 {
            ans = (ans + Self::dfs(n - 1, square, &mut memo)) % Self::MOD;
        }
        return ans;
    }
    fn dfs(remain: usize, square: usize, memo: &mut Vec<Vec<Option<i32>>>) -> i32 {
        if remain == 0 {
            return 1;
        }
        if let Some(m) = memo[remain][square] {
            return m;
        }
        let mut res: i32 = 0;
        for &square_nxt in Self::JUMPS[square] {
            res = (res + Self::dfs(remain - 1, square_nxt as usize, memo)) % Self::MOD;
        }
        memo[remain][square] = Some(res);
        return res;
    }
}
