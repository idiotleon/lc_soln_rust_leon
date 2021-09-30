/// @author: Leon
/// https://leetcode.com/problems/maximum-number-of-words-you-can-type/
///
/// Time Complexity:    O(`len_t`)
/// Space Complexity:   O(1)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        // not used
        // let len_t = text.len();

        const CH_SPACE: char = ' ';

        let bit_mask: u32 = {
            let mut tmp = 0;
            for ch in broken_letters.chars() {
                tmp = tmp | (1 << (ch as u8 - 'a' as u8));
            }
            tmp
        };

        let words = text.split(CH_SPACE).collect::<Vec<&str>>();
        let mut cnt: i32 = 0;

        'outer: for word in words {
            for ch in word.chars() {
                if (bit_mask & (1 << (ch as u8 - 'a' as u8))) > 0 {
                    continue 'outer;
                }
            }

            cnt += 1
        }

        cnt
    }
}
