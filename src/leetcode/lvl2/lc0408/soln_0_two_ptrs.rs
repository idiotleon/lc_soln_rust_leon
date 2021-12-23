/// https://leetcode.com/problems/valid-word-abbreviation/
/// Time Complexity:    O(maxOf(`len_w`, `len_ab`))
/// Space Complexity:   O(1) / O(`len_w` + `len_ab`)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn valid_word_abbreviation(word: String, abbr: String) -> bool {
        let len_w = word.len();
        let len_ab = abbr.len();
        let chs_w: Vec<char> = word.chars().collect();
        let chs_ab: Vec<char> = abbr.chars().collect();
        let mut number: u8 = 0;
        let mut idx_w: usize = 0;
        let mut idx_ab: usize = 0;
        while idx_w < len_w && idx_ab < len_ab {
            if chs_ab[idx_ab].is_digit(10) {
                number = number * 10 + (chs_ab[idx_ab] as u8 - '0' as u8);
                if number == 0 {
                    return false;
                }
                idx_ab += 1;
            } else {
                idx_w += number as usize;
                if idx_w >= len_w || chs_w[idx_w] != chs_ab[idx_ab] {
                    return false;
                }
                number = 0;
                idx_w += 1;
                idx_ab += 1;
            }
        }
        idx_w += number as usize;
        idx_w == len_w && idx_ab == len_ab
    }
}
