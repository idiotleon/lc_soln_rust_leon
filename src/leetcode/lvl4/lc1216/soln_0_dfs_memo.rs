use std::cmp::max;

/// @author: Leon
/// https://leetcode.com/problems/valid-palindrome-iii/
/// Time Complexity:    O(`len_s` ^ 2)
/// Space Complexity:   O(`len_s` ^ 2)
/// Reference:
/// https://leetcode.com/problems/valid-palindrome-iii/discuss/397606/Find-Longest-Palindromic-Subsequence.
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_valid_palindrome(s: String, k: i32) -> bool {
        let len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        let mut memo: Vec<Vec<Option<i32>>> = vec![vec![None; len_s]; len_s];
        len_s as i32 - Self::dfs(0, len_s - 1, &chs, &mut memo) <= k
    }
    fn dfs(lo: usize, hi: usize, chs: &Vec<char>, memo: &mut Vec<Vec<Option<i32>>>) -> i32 {
        if lo > hi {
            return 0;
        }
        if lo == hi {
            return 1;
        }
        if let Some(m) = memo[lo][hi] {
            return m;
        }
        let longest = if chs[lo] == chs[hi] {
            Self::dfs(lo + 1, hi - 1, chs, memo) + 2
        } else {
            max(
                Self::dfs(lo + 1, hi, chs, memo),
                Self::dfs(lo, hi - 1, chs, memo),
            )
        };
        memo[lo][hi] = Some(longest);
        longest
    }
}
