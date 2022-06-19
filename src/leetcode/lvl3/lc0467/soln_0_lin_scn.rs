/// @author: Leon
/// https://leetcode.com/problems/unique-substrings-in-wraparound-string/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/unique-substrings-in-wraparound-string/discuss/95439/Concise-Java-solution-using-DP
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_substring_in_wrapround_string(s: String) -> i32 {
        let len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        let mut freqs: Vec<u16> = vec![0; 26];
        // the running length
        let mut len: u16 = 0;
        for idx in 0..len_s {
            if idx > 0 && ((chs[idx] as i8 - chs[idx - 1] as i8 == 1) || (chs[idx - 1] as i8 - chs[idx] as i8 == 25)){
                len += 1;
            }else{
                len = 1;
            }
            let idx_ch: usize = chs[idx] as usize - 'a' as usize;
            freqs[idx_ch] = std::cmp::max(freqs[idx_ch], len);
        }
        freqs.into_iter().sum::<u16>() as i32
    }
}
