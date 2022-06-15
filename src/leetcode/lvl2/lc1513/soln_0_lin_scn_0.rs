/// @author: Leon
/// https://leetcode.com/problems/number-of-substrings-with-only-1s/
/// Time Complexity:    O(`_len_s`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/number-of-substrings-with-only-1s/discuss/731580/JavaC%2B%2BPython-Count
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_sub(s: String) -> i32 {
        const MOD: i32 = 1e9 as i32 + 7;
        const _ZERO: char = '0';
        const ONE: char = '1';
        let _len_s: usize = 0;
        // the running length
        let mut len: i32 = 0;
        let mut ans: i32 = 0;
        for ch in s.chars() {
            if ch == ONE {
                len += 1;
            } else {
                len = 0;
            }
            ans = (ans + len) % MOD;
        }
        ans
    }
}
