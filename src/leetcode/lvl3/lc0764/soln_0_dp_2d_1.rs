use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/largest-plus-sign/
/// Time Complexity:    O(`n` ^ 2)
/// Space Complexity:   O(`n` ^ 2)
/// Reference:
/// https://leetcode.com/problems/largest-plus-sign/editorial/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn order_of_largest_plus_sign(n: i32, mines: Vec<Vec<i32>>) -> i32 {
        let len_ms: usize = mines.len();
        let banned: HashSet<i32> = {
            let mut set: HashSet<i32> = HashSet::with_capacity(len_ms);
            for mine in mines {
                set.insert(mine[0] * n + mine[1]);
            }
            set
        };
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n as usize]; n as usize];
        let mut longest: i32 = 0;
        let mut order: i32;
        for r in 0..n {
            order = 0;
            for c in 0..n {
                order = if banned.contains(&(r * n + c)) {
                    0
                } else {
                    1 + order
                };
                dp[r as usize][c as usize] = order;
            }
            order = 0;
            for c in (0..n).rev() {
                order = if banned.contains(&(r * n + c)) {
                    0
                } else {
                    1 + order
                };
                dp[r as usize][c as usize] = std::cmp::min(dp[r as usize][c as usize], order);
            }
        }
        for c in 0..n {
            order = 0;
            for r in 0..n {
                order = if banned.contains(&(r * n + c)) {
                    0
                } else {
                    1 + order
                };
                dp[r as usize][c as usize] = std::cmp::min(dp[r as usize][c as usize], order);
            }
            order = 0;
            for r in (0..n).rev() {
                order = if banned.contains(&(r * n + c)) {
                    0
                } else {
                    1 + order
                };
                dp[r as usize][c as usize] = std::cmp::min(dp[r as usize][c as usize], order);
                longest = std::cmp::max(longest, dp[r as usize][c as usize]);
            }
        }
        return longest;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let n: i32 = 5;
        let mines: Vec<Vec<i32>> = vec![vec![4, 2]];
        let expected: i32 = 2;
        let actual: i32 = Solution::order_of_largest_plus_sign(n, mines);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let n: i32 = 1;
        let mines: Vec<Vec<i32>> = vec![vec![0, 0]];
        let expected: i32 = 0;
        let actual: i32 = Solution::order_of_largest_plus_sign(n, mines);
        assert_eq!(expected, actual);
    }
}
