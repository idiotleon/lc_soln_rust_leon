/// @author: Leon
/// https://leetcode.com/problems/richest-customer-wealth/
/// Time Complexity:    O(`len_r` * `len_c`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let _len_r: usize = accounts.len();
        let _len_c: usize = accounts[0].len();
        let mut richest: i32 = 0;
        for row in accounts {
            richest = std::cmp::max(richest, row.into_iter().sum());
        }
        richest
    }
}
