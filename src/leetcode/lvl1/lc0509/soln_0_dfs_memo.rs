/// @author: Leon
/// https://leetcode.com/problems/fibonacci-number/
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(`RANGE`) ~ O(1)
/// Reference:
/// https://leetcode.com/problems/fibonacci-number/discuss/215992/Java-Solutions
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn fib(n: i32) -> i32 {
        const RANGE: usize = 30 + 1;
        let mut memo: Vec<Option<i32>> = vec![None; RANGE];
        Self::dfs(n, &mut memo)
    }
    fn dfs(num: i32, memo: &mut Vec<Option<i32>>) -> i32 {
        if num <= 1 {
            return num;
        }
        if let Some(n) = memo[num as usize] {
            return n;
        }
        Self::dfs(num - 1, memo) + Self::dfs(num - 2, memo)
    }
}
