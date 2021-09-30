/// @author: Leon
/// https://leetcode.com/problems/check-if-the-sentence-is-pangram/
///
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(26) ~ O(1)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        // not used
        // let len_s = sentence.len();

        let mut freqs: Vec<u16> = vec![0; 26];
        let mut cnt = 0;

        for ch in sentence.chars() {
            let idx = ch as usize - 'a' as usize;
            if freqs[idx] == 0 {
                cnt += 1;
            }
            freqs[idx] += 1;
        }

        cnt == 26
    }
}
