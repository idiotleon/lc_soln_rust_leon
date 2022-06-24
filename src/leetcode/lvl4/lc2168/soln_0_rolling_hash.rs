use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/unique-substrings-with-equal-digit-frequency/
/// Time Complexity:    O(`len_s` ^ 2)
/// Space Complexity:   O(`len_s`)
/// Reference:
/// https://leetcode.com/problems/unique-substrings-with-equal-digit-frequency/discuss/1759403/Java-rolling-hash-13-lines-+-trie/1267819
/// https://leetcode.com/problems/unique-substrings-with-equal-digit-frequency/discuss/1759403/Java-rolling-hash-13-lines-%2B-trie
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn equal_digit_frequency(s: String) -> i32 {
        let len_s: usize = s.len();
        let bs: &[u8] = s.as_bytes();
        const MOD: u32 = 1e9 as u32 + 7;
        // there are only 10 digits in decimal
        const RANGE: u32 = 10 + 1;
        let mut seen: HashSet<u32> = HashSet::new();
        for lo in 0..len_s {
            let mut freqs: Vec<u16> = vec![0; 10];
            let mut unique: u16 = 0;
            let mut most: u16 = 0;
            let mut hash: u32 = 0;
            for hi in lo..len_s {
                let digit: usize = (bs[hi] - b'0') as usize;
                if freqs[digit] == 0 {
                    unique += 1;
                }
                freqs[digit] += 1;
                most = std::cmp::max(most, freqs[digit]);
                hash = (hash * RANGE + digit as u32 + 1) % MOD;
                if (most * unique) as usize == hi - lo + 1 {
                    seen.insert(hash);
                }
            }
        }
        seen.len() as i32
    }
}
