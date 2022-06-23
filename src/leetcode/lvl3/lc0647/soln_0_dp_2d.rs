/// @author: Leon
/// https://leetcode.com/problems/palindromic-substrings/
/// Time Complexity:     O()
/// Space Complexity:    O()
/// Reference:
/// https://leetcode.com/problems/palindromic-substrings/discuss/258917/Java-Simple-Code%3A-DP-short
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_substrings(str: String) -> i32 {
        let len_s: usize = str.len();
        let mut dp: Vec<Vec<bool>> = vec![vec![false; len_s]; len_s];
        let mut count: i32 = 0;
        for len in 0..len_s {
            for lo in 0..len_s - len {
                let hi = lo + len;
                if str.chars().nth(lo).unwrap() == str.chars().nth(hi).unwrap() {
                    dp[lo][hi] = if hi - lo + 1 <= 3 {
                        true
                    } else {
                        dp[lo + 1][hi - 1]
                    };
                }
                if dp[lo][hi] {
                    count += 1;
                }
            }
        }
        return count;
    }
}
