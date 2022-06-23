/// @author: Leon
/// https://leetcode.com/problems/longest-palindromic-subsequence/
/// Time Complexity:    O(`len_s` ^ 2)
/// Space Complexity:   O(`len_s` ^ 2)
/// Reference:
/// https://leetcode.com/problems/longest-palindromic-subsequence/discuss/99101/Straight-forward-Java-DP-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let len_s: usize = s.len();
        let mut memo = vec![vec![-1; len_s]; len_s];
        let chs: Vec<char> = s.chars().collect();
        Self::dfs(0, len_s - 1, &chs, &mut memo)
    }
    fn dfs(lo: usize, hi: usize, chs: &Vec<char>, memo: &mut Vec<Vec<i32>>) -> i32 {
        if memo[lo][hi] >= 0 {
            return memo[lo][hi];
        }
        if lo > hi {
            return 0;
        }
        if lo == hi {
            return 1;
        }
        memo[lo][hi] = if chs[lo] == chs[hi] {
            Self::dfs(lo + 1, hi - 1, chs, memo) + 2
        } else {
            std::cmp::max(
                Self::dfs(lo + 1, hi, chs, memo),
                Self::dfs(lo, hi - 1, chs, memo),
            )
        };
        memo[lo][hi]
    }
}
