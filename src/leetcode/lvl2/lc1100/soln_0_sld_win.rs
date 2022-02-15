/// @author: Leon
/// https://leetcode.com/problems/find-k-length-substrings-with-no-repeated-characters/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_k_len_substr_no_repeats(s: String, k: i32) -> i32 {
        let len_s: usize = s.len();
        if k as usize > len_s {
            return 0;
        }
        let k: u16 = k as u16;
        let mut freqs: Vec<u16> = vec![0; 26];
        let mut cnt: u16 = 0;
        let mut distinct: u16 = 0;
        let chs: Vec<char> = s.chars().collect();
        for idx in 0..len_s {
            let freq_hi = &mut freqs[chs[idx] as usize - 'a' as usize];
            if *freq_hi == 0 {
                distinct += 1;
            }
            *freq_hi += 1;
            if distinct == k {
                cnt += 1;
            }
            if idx + 1 >= k as usize {
                let freq_lo = &mut freqs[chs[idx + 1 - k as usize] as usize - 'a' as usize];
                *freq_lo -= 1;
                if *freq_lo == 0 {
                    distinct -= 1;
                }
            }
        }
        cnt as i32
    }
}
