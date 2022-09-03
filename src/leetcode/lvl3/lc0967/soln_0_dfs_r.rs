/// @author: Leon
/// https://leetcode.com/problems/numbers-with-same-consecutive-differences/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/numbers-with-same-consecutive-differences/discuss/211193/C%2B%2B-DFS
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut ans: Vec<i32> = Vec::with_capacity(k as usize);
        for num in 1..=9 {
            Self::dfs(num, n - 1, k, &mut ans);
        }
        return ans;
    }
    fn dfs(num: i32, n: i32, k: i32, res: &mut Vec<i32>) {
        if n == 0 {
            res.push(num);
            return;
        }
        let digit = num % 10;
        if digit + k <= 9 {
            Self::dfs(num * 10 + digit + k, n - 1, k, res);
        }
        if k != 0 && digit >= k {
            Self::dfs(num * 10 + digit - k, n - 1, k, res);
        }
    }
}
