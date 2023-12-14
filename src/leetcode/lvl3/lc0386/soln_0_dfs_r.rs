/// @author: Leon
/// https://leetcode.com/problems/lexicographical-numbers/
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(10) ~ O(1)
/// Reference:
/// https://leetcode.com/problems/lexicographical-numbers/solutions/86231/simple-java-dfs-solution/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::new();
        for digit in 1..10 {
            Self::dfs(digit, n, &mut ans);
        }
        return ans;
    }
    fn dfs(cur: i32, n: i32, res: &mut Vec<i32>) {
        if cur > n {
            return;
        }
        res.push(cur);
        for digit in 0..10 {
            let nxt: i32 = 10 * cur + digit;
            if nxt > n {
                return;
            }
            Self::dfs(nxt, n, res);
        }
    }
}
