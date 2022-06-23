use super::nested_integer::NestedInteger;

/// @author: Leon
/// https://leetcode.com/problems/nested-list-weight-sum/
/// Time Complexity:    O(N)
/// Space Complexity:   O(H)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn depth_sum(nested_list: Vec<NestedInteger>) -> i32 {
        let _len_ni: usize = nested_list.len();
        let mut sum: i32 = 0;
        for ni in nested_list.into_iter() {
            sum += Self::dfs(&ni, 1);
        }
        sum
    }
    fn dfs(ni: &NestedInteger, depth: u8) -> i32 {
        let mut sum: i32 = 0;
        match ni {
            NestedInteger::Int(num) => sum += num * depth as i32,
            NestedInteger::List(nis) => {
                for ni in nis {
                    sum += Self::dfs(ni, 1 + depth);
                }
            }
        }
        sum
    }
}
