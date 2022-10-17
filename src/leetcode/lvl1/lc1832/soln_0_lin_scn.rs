/// @author: Leon
/// https://leetcode.com/problems/check-if-the-sentence-is-pangram/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(26) ~ O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut freqs: Vec<u16> = vec![0; 26];
        let mut cnt: u8 = 0;
        for ch in sentence.chars() {
            let freq = &mut freqs[ch as usize - 'a' as usize];
            if *freq == 0 {
                cnt += 1;
                if cnt == 26 {
                    return true;
                }
            }
            *freq += 1;
        }
        return false;
    }
}
