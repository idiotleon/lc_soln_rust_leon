// https://leetcode.com/problems/palindromic-substrings/
//
// Time Complexity:     O()
// Space Complexity:    O()
//
// Reference:
// https://leetcode.com/problems/palindromic-substrings/discuss/258917/Java-Simple-Code%3A-DP-short
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_substrings(str: String) -> i32 {
        const LEN_S: usize = str.len();
        let mut dp = [[false; LEN_S]; LEN_S];
        let mut count: i32 = 0;

        for len in 0..LEN_S {
            for lo in 0..LEN_S - len {
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
