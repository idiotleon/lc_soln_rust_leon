/// @author: Leon
/// https://leetcode.com/problems/longest-substring-without-repeating-characters/
/// Time Complexity:    O(`len_s`)
/// Space Compleixty:   O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        let mut freqs: Vec<u8> = vec![0; 128];
        let mut lo: usize = 0;
        let mut hi: usize = 0;
        let mut longest: usize = 0;
        while hi < len_s {
            freqs[chs[hi] as usize] += 1;
            while freqs[chs[hi] as usize] > 1 {
                freqs[chs[lo] as usize] -= 1;
                lo += 1;
            }
            let len = hi - lo + 1;
            longest = std::cmp::max(longest, len);
            hi += 1;
        }
        longest as i32
    }
}
