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
/// Note:
/// this is NOT a correct solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn cnt_bkfst(n: i32) -> i32 {
        let mut dp: Vec<Vec<i32>> = {
            let mut dp = vec![vec![0; 3]; n as usize + 1];
            dp[1][1] = 1;
            dp[1][1] = 1;
            dp[2][1] = 1;
            dp[1][2] = 1;
            dp[2][2] = 1;
            dp[3][2] = 1;
            dp
        };
        let mut most: i32 = 0;
        for cur in 1..=n as usize {
            dp[cur][0] = 1 + dp[cur - 1][0];
            if cur > 2 {
                dp[cur][1] = 1 + dp[cur - 2][1];
            }
            if cur > 3 {
                dp[cur][2] = 1 + dp[cur - 3][2];
            }
            most = std::cmp::max(most, dp[cur].iter().sum());
        }
        most
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_test_case_0() {
        let actual = Solution::cnt_bkfst(3);
        let expected = 6;
        assert_eq!(expected, actual);
    }
}
