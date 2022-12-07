/// @author: Leon
/// https://leetcode.com/problems/single-row-keyboard/description/
/// Time Complexity:    O(`_len_kb` + `_len_wd`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn calculate_time(keyboard: String, word: String) -> i32 {
        let _len_kb: usize = keyboard.len();
        let _len_wd: usize = word.len();
        let dict: Vec<i8> = {
            let mut dict: Vec<i8> = vec![0; 26];
            for (idx, ch) in keyboard.chars().enumerate() {
                dict[ch as usize - 'a' as usize] = idx as i8;
            }
            dict
        };
        let mut sum: i32 = 0;
        let mut idx_prev: i8 = 0;
        for ch in word.chars() {
            let idx = ch as usize - 'a' as usize;
            sum += (idx_prev - dict[idx]).abs() as i32;
            idx_prev = dict[idx];
        }
        return sum;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let keyboard: String = "abcdefghijklmnopqrstuvwxyz".to_owned();
        let word: String = "cba".to_owned();
        let expected: i32 = 4;
        let actual = Solution::calculate_time(keyboard, word);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_sample_input_2_should_return_expected() {
        let keyboard: String = "pqrstuvwxyzabcdefghijklmno".to_owned();
        let word: String = "leetcode".to_owned();
        let expected: i32 = 73;
        let actual = Solution::calculate_time(keyboard, word);
        assert_eq!(expected, actual);
    }
}
