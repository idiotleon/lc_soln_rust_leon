/// @author: Leon
/// https://leetcode.com/problems/ugly-number/description/
/// Time Complexity:    O(lg(`n`))
/// Space Complexity:   O(lg(`n`))
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        return Self::dfs(n);
    }
    fn dfs(num: i32) -> bool {
        if num <= 0 {
            return false;
        }
        if num == 1 {
            return true;
        }
        if num % 5 == 0 {
            return Self::dfs(num / 5);
        }
        if num % 3 == 0 {
            return Self::dfs(num / 3);
        }
        if num % 2 == 0 {
            return Self::dfs(num / 2);
        }
        return false;
    }
}
