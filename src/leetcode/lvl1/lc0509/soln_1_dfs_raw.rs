/// @author: Leon
/// https://leetcode.com/problems/fibonacci-number/
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(`_RANGE`) ~ O(1)
/// Reference:
/// https://leetcode.com/problems/fibonacci-number/discuss/215992/Java-Solutions
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn fib(n: i32) -> i32 {
        const _RANGE: usize = 30 + 1;
        Self::dfs(n)
    }
    fn dfs(num: i32) -> i32 {
        if num <= 1 {
            return num;
        }
        Self::dfs(num - 1) + Self::dfs(num - 2)
    }
}
