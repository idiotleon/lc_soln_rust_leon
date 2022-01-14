/// https://leetcode.com/problems/string-to-integer-atoi/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(`len_s`)
/// Reference;
/// https://leetcode.com/problems/string-to-integer-atoi/discuss/4654/My-simple-solution/5520
/// https://leetcode.com/problems/string-to-integer-atoi/discuss/4654/My-simple-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }
        const SPACE: char = ' ';
        const PLUS: char = '+';
        const MINUS: char = '-';
        let len_s: usize = s.len();
        let chs: Vec<char> = s.chars().collect();
        let mut idx: usize = 0;
        while idx < len_s && chs[idx] == SPACE {
            idx += 1;
        }
        if idx == len_s {
            return 0;
        }
        let sign: i8 = {
            let mut sign: i8 = 1;
            if chs[idx] == MINUS || chs[idx] == PLUS {
                sign = if chs[idx] == MINUS { -1 } else { 1 };
                idx += 1
            }
            sign
        };
        let mut base: i32 = 0;
        while idx < len_s && chs[idx].is_digit(10) {
            if base > i32::MAX / 10 || (base == i32::MAX / 10 && (chs[idx] as u8 - '0' as u8) > 7) {
                return if sign == 1 { i32::MAX } else { i32::MIN };
            }
            base = 10 * base + (chs[idx] as i32 - '0' as i32);
            idx += 1;
        }
        base * sign as i32
    }
}
