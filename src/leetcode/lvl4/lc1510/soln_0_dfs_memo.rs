/// @author: Leon
/// https://leetcode.com/problems/stone-game-iv/
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(`n`)
/// Reference:
/// https://leetcode.com/problems/stone-game-iv/discuss/730490/Java-or-Heavily-Commented-or-Subproblems-Visualised
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        let mut memo: Vec<Option<bool>> = vec![None; n as usize + 1];
        Self::dfs(n, &mut memo)
    }
    fn dfs(n: i32, memo: &mut Vec<Option<bool>>) -> bool {
        if let Some(m) = memo[n as usize] {
            return m;
        }
        let mut can_reach: bool = false;
        let mut remove: i32 = 1;
        while n - remove * remove >= 0 {
            if n - remove * remove == 0 {
                can_reach = true;
                break;
            } else {
                can_reach = can_reach || !Self::dfs(n - remove * remove, memo)
            }
            remove += 1;
        }
        memo[n as usize] = Some(can_reach);
        can_reach
    }
}
