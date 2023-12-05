/// @author: Leon
/// https://leetcode.com/problems/count-of-matches-in-tournament/
/// Time Complexity:    O(lg(`n`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        let mut n: i32 = n;
        let mut cnt: i32 = 0;
        while n > 1 {
            cnt += n / 2;
            n = if n % 2 == 0 { n / 2 } else { (n - 1) / 2 + 1 }
        }
        return cnt;
    }
}
