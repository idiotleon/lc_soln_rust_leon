/// @author: Leon
/// https://leetcode.com/problems/maximum-number-of-words-you-can-type/
/// Time Complexity:    O(`_len_t`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let _len_t = text.len();
        const CH_SPACE: char = ' ';
        let freqs: Vec<u8> = {
            let mut tmp = vec![0 as u8; 26];
            for ch in broken_letters.chars() {
                tmp[ch as usize - 'a' as usize] += 1;
            }
            tmp
        };
        let words = text.split(CH_SPACE).collect::<Vec<&str>>();
        let mut cnt: i32 = 0;
        'outer: for word in words {
            for ch in word.chars() {
                if freqs[ch as usize - 'a' as usize] > 0 {
                    continue 'outer;
                }
            }
            cnt += 1;
        }
        cnt
    }
}
