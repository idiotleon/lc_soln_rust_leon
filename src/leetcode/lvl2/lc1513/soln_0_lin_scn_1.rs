/// @author: Leon
/// https://leetcode.com/problems/number-of-substrings-with-only-1s/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
/// note:
/// a good lession to learn about integer overflow
struct Solution;

#[allow(dead_code)]
impl Solution {
    const MOD: i64 = 1e9 as i64 + 7;
    pub fn num_sub(s: String) -> i32 {
        const ZERO: char = '0';
        const _ONE: char = '1';
        let _len_s: usize = 0;
        // the running length
        let mut len: i64 = 0;
        let mut ans: i64 = 0;
        for ch in s.chars() {
            if ch == ZERO {
                ans = (ans + Self::get_count(len)) % Self::MOD;
                len = 0;
            } else {
                len += 1;
            }
        }
        ((ans + Self::get_count(len)) % Self::MOD) as i32
    }
    fn get_count(len: i64) -> i64 {
        ((len as i64) * (len as i64 + 1) / 2) % Self::MOD
    }
}
