use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/decode-the-message/
/// Time Complexity:    O(`_len_k`) + O(`_len_m`) ~ O(max(`_len_k`. `_len_m`))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/decode-the-message/discuss/2229841/Rust-Linear-Scan/1470667
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let _len_k: usize = key.len();
        let _len_m: usize = message.len();
        const SPACE: char = ' ';
        let dict: HashMap<char, char> = {
            let mut map: HashMap<char, char> = HashMap::with_capacity(26);
            let mut iter_all_letters = 'a'..='z';
            for ch in key.chars().filter(|&c| c != SPACE) {
                map.entry(ch)
                    .or_insert_with(|| iter_all_letters.next().unwrap());
            }
            map
        };
        message
            .chars()
            .map(|c| *dict.get(&c).unwrap_or(&c))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let key: String = "the quick brown fox jumps over the lazy dog".to_owned();
        let message: String = "vkbs bs t suepuv".to_owned();
        let expected: String = "this is a secret".to_owned();
        let actual: String = Solution::decode_message(key, message);
        assert_eq!(expected, actual);
    }
}
