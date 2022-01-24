/// @author: Leon
/// https://leetcode.com/problems/stone-game-iv/
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(`n`)
/// Reference:
/// https://leetcode.com/problems/stone-game-iv/discuss/730582/JavaC++Python-DP/619672
/// https://leetcode.com/problems/stone-game-iv/discuss/730582/JavaC%2B%2BPython-DP
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let mut dp: Vec<bool> = vec![false; n as usize + 1];
        for idx in 0..=n as usize {
            if dp[idx] {
                continue;
            }
            let mut remove = 1;
            while idx as i32 + remove * remove <= n {
                dp[idx + remove as usize * remove as usize] = true;
                remove += 1;
            }
        }
        dp[n as usize]
    }
}
