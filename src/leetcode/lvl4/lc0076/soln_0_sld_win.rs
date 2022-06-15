/// @author: Leon
/// https://leetcode.com/problems/minimum-window-substring/
/// Time Complexity:    O(max(`len_s`, `len_t`))
/// Space Complexity:   O(`len_s`)
/// Note:
/// please be sensitive to whether all letters are all lower-cases only
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let len_s: usize = s.len();
        let len_t: usize = t.len();
        // sanity check, required
        if len_s < len_t {
            return "".to_owned();
        }
        let mut cnt: u16 = len_t as u16;
        let chs: Vec<char> = s.chars().collect();
        let mut shortest: &str = &"";
        let mut freqs: Vec<i32> = {
            let mut freqs: Vec<i32> = vec![0; 128];
            for ch in t.chars() {
                freqs[ch as usize] += 1;
            }
            freqs
        };
        let mut hi: usize = 0;
        let mut lo: usize = 0;
        while hi < len_s {
            let idx_ch_hi: usize = chs[hi] as usize;
            if freqs[idx_ch_hi] > 0 {
                cnt -= 1;
            }
            freqs[idx_ch_hi] -= 1;
            while cnt == 0 {
                if shortest.is_empty() || (hi + 1 - lo) < shortest.len() {
                    shortest = &s[lo..=hi];
                }
                let idx_ch_lo: usize = chs[lo] as usize;
                freqs[idx_ch_lo] += 1;
                if freqs[idx_ch_lo] > 0 {
                    cnt += 1;
                }
                lo += 1;
            }
            hi += 1;
        }
        shortest.to_owned()
    }
}
