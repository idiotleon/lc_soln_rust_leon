/// @author: Leon
/// https://leetcode.com/problems/check-if-word-equals-summation-of-two-words/
/// Time Complexity:    O(`_len_rw` + `_len_sw`) + O(`_len_tw`) ~ O(max(`_len_rw`, `_len_sw, `_lentw`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        let _len_rw: usize = first_word.len();
        let _len_sw: usize = second_word.len();
        let _len_tw: usize = target_word.len();
        Self::str_to_num(first_word) + Self::str_to_num(second_word)
            == Self::str_to_num(target_word)
    }
    fn str_to_num(word: String) -> u8 {
        let mut num: u8 = 0;
        for ch in word.chars() {
            num = num * 10 + (ch as u8 - 'a' as u8);
        }
        num
    }
}
