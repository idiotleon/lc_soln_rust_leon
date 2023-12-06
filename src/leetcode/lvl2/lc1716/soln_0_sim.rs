/// @author: Leon
/// https://leetcode.com/problems/calculate-money-in-leetcode-bank/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/calculate-money-in-leetcode-bank/editorial/?envType=daily-question&envId=2023-12-06
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut ans: i32 = 0;
        let mut monday = 1;
        let mut n = n;
        while n > 0 {
            for day in 0..std::cmp::min(n, 7) {
                ans += monday + day;
            }
            n -= 7;
            monday += 1;
        }
        return ans;
    }
}
