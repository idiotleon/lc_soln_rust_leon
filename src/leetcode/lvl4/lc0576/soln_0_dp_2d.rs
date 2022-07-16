/// @author: Leon
/// https://leetcode.com/problems/out-of-boundary-paths/
/// Time Complexity:    O(`m` * `n` * `max_move`)
/// Space Complexity:   O(`m` * `n`)
/// Reference:
/// https://leetcode.com/problems/out-of-boundary-paths/discuss/102967/Java-Solution-DP-with-space-compression
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        if max_move == 0 {
            return 0;
        }
        let len_rs: isize = m as isize;
        let len_cs: isize = n as isize;
        const DIRS: &[isize] = &[0, -1, 0, 1, 0];
        const MOD: i32 = 1e9 as i32 + 7;
        let mut counts: Vec<Vec<i32>> = {
            let mut counts: Vec<Vec<i32>> = vec![vec![0; len_cs as usize]; len_rs as usize];
            counts[start_row as usize][start_column as usize] = 1;
            counts
        };
        let mut cnt: i32 = 0;
        for _ in 0..max_move {
            let mut dp: Vec<Vec<i32>> = vec![vec![0; len_cs as usize]; len_rs as usize];
            for r in 0..len_rs {
                for c in 0..len_cs {
                    for d in 0..4 {
                        let r_nxt: isize = r + DIRS[d];
                        let c_nxt: isize = c + DIRS[d + 1];
                        if r_nxt < 0 || c_nxt < 0 || r_nxt >= len_rs || c_nxt >= len_cs {
                            cnt = (cnt + counts[r as usize][c as usize]) % MOD;
                        } else {
                            dp[r_nxt as usize][c_nxt as usize] = (dp[r_nxt as usize]
                                [c_nxt as usize]
                                + counts[r as usize][c as usize])
                                % MOD;
                        }
                    }
                }
            }
            counts = dp;
        }
        cnt
    }
}
