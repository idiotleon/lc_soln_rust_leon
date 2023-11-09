/// @author: Leon
/// https://leetcode.com/problems/count-number-of-homogenous-substrings/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`) / O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        let mut ans: i32 = 0;
        let mut len: i32 = 0;
        const MOD: i32 = 1e9 as i32 + 7;
        for idx in 0..len_s {
            if idx == 0 || chs[idx] == chs[idx - 1] {
                len += 1;
            } else {
                len = 1;
            }
            ans = (ans + len) % MOD;
        }
        return ans;
    }
}
