/// https://leetcode.com/problems/unique-binary-search-trees/
/// Time Complexity:     O(`n` ^ 2)
/// Space Complexity:    O(`n`)
/// References:
/// https://leetcode.com/problems/unique-binary-search-trees/discuss/31696/Simple-Recursion-Java-Solution-with-Explanation/30507
/// https://leetcode.com/problems/unique-binary-search-trees/discuss/31696/Simple-Recursion-Java-Solution-with-Explanation
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        let n: usize = n as usize;
        let mut memo: Vec<Option<i32>> = vec![None; n + 1];
        Self::dfs(n, &mut memo)
    }
    fn dfs(n: usize, memo: &mut Vec<Option<i32>>) -> i32 {
        if let Some(m) = memo[n] {
            return m;
        }
        if n <= 1 {
            return 1;
        }
        let mut amount = 0;
        for i in 1..=n {
            amount += Self::dfs(i - 1, memo) * Self::dfs(n - i, memo);
        }
        memo[n] = Some(amount);
        amount
    }
}
