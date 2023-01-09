/// @author: Leon
/// https://leetcode.com/problems/minimum-health-to-beat-game/
/// Time Complexity:    O(`_len_gs`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_health(damages: Vec<i32>, armor: i32) -> i64 {
        let _len_gs: usize = damages.len();
        let (sum, max): (i64, i32) = {
            let mut sum: i64 = 0;
            let mut max: i32 = 0;
            for damage in damages {
                max = std::cmp::max(max, damage);
                sum += damage as i64;
            }
            (sum, max)
        };
        return 1 + sum - std::cmp::min(max, armor) as i64;
    }
}
