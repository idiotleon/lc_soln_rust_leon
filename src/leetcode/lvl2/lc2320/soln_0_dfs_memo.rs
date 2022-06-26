/// @author: Leon
/// https://leetcode.com/problems/count-number-of-ways-to-place-houses/
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(`n`)
/// Reference:
/// https://leetcode.com/problems/count-number-of-ways-to-place-houses/discuss/2198118/Easy-C++-with-explanation-or-DP/1456238
/// https://leetcode.com/problems/count-number-of-ways-to-place-houses/discuss/2198118/Easy-C%2B%2B-with-explanation-or-DP
struct Solution;

#[allow(dead_code)]
impl Solution {
    const MOD: i64 = 1e9 as i64 + 7;
    pub fn count_house_placements(n: i32) -> i32 {
        let mut memo: Vec<Option<i64>> = vec![None; n as usize];
        Self::dfs(n - 1, &mut memo);
        let total: i64 = memo[n as usize - 1].unwrap();
        ((total * total) % Self::MOD) as i32
    }
    fn dfs(n: i32, memo: &mut Vec<Option<i64>>) -> i64 {
        if n < 0 {
            return 1;
        }
        if let Some(m) = memo[n as usize] {
            return m;
        }
        let place = Self::dfs(n - 2, memo);
        let not_place = Self::dfs(n - 1, memo);
        let total = (place + not_place) % Self::MOD;
        memo[n as usize] = Some(total);
        total
    }
}
