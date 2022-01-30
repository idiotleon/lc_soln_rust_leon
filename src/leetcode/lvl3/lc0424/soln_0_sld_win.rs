/// @author: Leon
/// https://leetcode.com/problems/longest-repeating-character-replacement/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
/// Reference:
/// https://leetcode.com/problems/longest-repeating-character-replacement/discuss/91271/Java-12-lines-O(n)-sliding-window-solution-with-explanation
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let len_s: usize = s.len();
        let chs_s: Vec<char> = s.chars().collect();
        let mut hi: usize = 0;
        let mut lo: usize = 0;
        let mut freqs: Vec<u16> = vec![0; 26];
        let mut longest: usize = 0;
        let mut most: u16 = 0;
        while hi < len_s {
            let freq_hi = &mut freqs[chs_s[hi] as usize - 'A' as usize];
            *freq_hi += 1;
            most = std::cmp::max(most, *freq_hi);
            while hi as u16 - lo as u16 + 1 - most > k as u16 {
                freqs[chs_s[lo] as usize - 'A' as usize] -= 1;
                lo += 1;
            }
            longest = std::cmp::max(longest, hi - lo + 1);
            hi += 1;
        }
        longest as i32
    }
}
