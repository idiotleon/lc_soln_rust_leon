/// @author: Leon
/// https://leetcode.com/problems/number-of-substrings-containing-all-three-characters/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/number-of-substrings-containing-all-three-characters/discuss/516977/JavaC%2B%2BPython-Easy-and-Concise
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        let mut freqs: Vec<u16> = vec![0; 3];
        let mut ans: i32 = 0;
        let mut cnt: u8 = 0;
        let mut lo: usize = 0;
        let mut hi: usize = 0;
        while hi < len_s {
            let idx_ch_hi = chs[hi] as usize - 'a' as usize;
            if freqs[idx_ch_hi] == 0 {
                cnt += 1;
            }
            freqs[idx_ch_hi] += 1;
            while cnt == 3 {
                let idx_ch_lo: usize = chs[lo] as usize - 'a' as usize;
                freqs[idx_ch_lo] -= 1;
                if freqs[idx_ch_lo] == 0 {
                    cnt -= 1;
                }
                lo += 1;
            }
            ans += lo as i32;
            hi += 1;
        }
        ans
    }
}
