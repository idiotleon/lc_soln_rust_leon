/// @author: Leon
/// https://leetcode.com/problems/count-number-of-ways-to-place-houses/
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/count-number-of-ways-to-place-houses/discuss/2198118/Easy-C%2B%2B-with-explanation-or-DP
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_house_placements(n: i32) -> i32 {
        let n: usize = n as usize;
        const MOD: i64 = 1e9 as i64 + 7;
        let mut house: i64 = 1;
        let mut space: i64 = 1;
        let mut total: i64 = house + space;
        for _ in 2..=n {
            house = space;
            space = total;
            total = (house + space) % MOD;
        }
        ((total * total) % MOD) as i32
    }
}
