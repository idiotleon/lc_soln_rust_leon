/// @author: Leon
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(`n`)
/// Explaination:
/// dp[cur][0], to have breakfast1 at the `cur` day
/// transition: dp[cur][0] = 1 + dp[cur - 1][0]
/// dp[cur][1], to have breakfast2 at the `cur` day
/// transiation: dp[cur][1] = 1 + dp[cur - 2][3]
/// dp[cur][2], to have breakfast3 at the `cur` day
/// transition: dp[cur][2] = 1 + dp[cur - 3][2]
/// Result:
/// the most overral
struct Solution;

#[allow(dead_code)]
impl Solution {
    // pub fn cnt_bkfst(n: i32) -> i32 {}
}

#[cfg(test)]
mod test {
    use soln_0_dp_2d::*;
    #[test]
    fn it_works_with_test_case_0() {
        let actual = Solution::cnt_bkfst(3);
        let expected = 6;
        assert_eq!(expected, actual);
    }
}
