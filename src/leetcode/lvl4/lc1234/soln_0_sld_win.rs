/// @author: Leon
/// https://leetcode.com/problems/replace-the-substring-for-balanced-string/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`) / O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn balanced_string(s: String) -> i32 {
        const RANGE: usize = 1e5 as usize + 7;
        let len_s: usize = s.len();
        let freq_exp: u32 = len_s as u32 / 4;
        let chs: Vec<char> = s.chars().collect();
        let mut shortest: usize = RANGE;
        let mut freqs: Vec<u32> = {
            let mut freqs: Vec<u32> = vec![0; 26];
            for &ch in &chs {
                freqs[ch as usize - 'A' as usize] += 1;
            }
            freqs
        };
        if Self::is_valid(&freqs, freq_exp) {
            return 0;
        }
        let mut hi: usize = 0;
        let mut lo: usize = 0;
        while hi < len_s {
            freqs[chs[hi] as usize - 'A' as usize] -= 1;
            while Self::is_valid(&freqs, freq_exp) {
                let len = hi - lo + 1;
                shortest = std::cmp::min(shortest, len);
                freqs[chs[lo] as usize - 'A' as usize] += 1;
                lo += 1
            }
            hi += 1;
        }
        shortest as i32
    }
    fn is_valid(freqs: &Vec<u32>, freq_exp: u32) -> bool {
        for &freq in freqs {
            if freq > freq_exp {
                return false;
            }
        }
        true
    }
}
