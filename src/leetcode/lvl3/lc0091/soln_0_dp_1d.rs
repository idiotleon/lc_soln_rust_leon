/// https://leetcode.com/problems/decode-ways/
///
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
///
/// Reference:
/// https://leetcode.com/problems/decode-ways/discuss/30358/Java-clean-DP-solution-with-explanation/330433
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let len_s = s.len();
        let chs: Vec<char> = s.chars().collect();
        let mut dp: Vec<u32> = vec![0; len_s];
        if chs[0] != '0' {
            dp[0] = 1;
        }
        for idx in 1..len_s {
            let cur = chs[idx] as u32 - '0' as u32;
            if cur != 0 {
                dp[idx] += dp[idx - 1];
            }
            let prev = (chs[idx - 1] as u32 - '0' as u32) * 10 + cur;
            if prev >= 10 && prev <= 26 {
                dp[idx] += if idx >= 2 { dp[idx - 2] } else { 1 }
            }
        }
        dp[len_s - 1] as i32
    }
}
