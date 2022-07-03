/// @author: Leon
/// https://leetcode.com/problems/decode-the-message/
/// Time Complexity:    O(`_len_k`) + O(`_len_m`) ~ O(max(`_len_k`, `_len_m`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let _len_k: usize = key.len();
        let _len_m: usize = message.len();
        const RANGE: u8 = 26 + 1;
        const IMPL: char = '#';
        const SPACE: char = ' ';
        let dict: Vec<u8> = {
            let mut dict: Vec<u8> = vec![RANGE; 26];
            let mut used: Vec<u8> = vec![0; 26];
            let mut distinct: u8 = 0;
            let mut idx_dict: usize = 0;
            for ch in key.chars() {
                if ch == SPACE {
                    continue;
                }
                // let idx_ch: i32 = ch as i32 - 'a' as i32;
                // let idx_ch: usize = idx_ch as usize;
                let idx_ch: usize = ch as usize - 'a' as usize;
                if used[idx_ch] == 0 {
                    distinct += 1;
                    used[idx_ch] += 1;
                    dict[idx_ch] = idx_dict as u8;
                    idx_dict += 1;
                }
                // slight optimization
                if distinct == 26 {
                    break;
                }
            }
            dict
        };
        let mut ans: String = "".to_owned();
        for ch in message.chars() {
            if ch == SPACE {
                ans.push(SPACE);
                continue;
            }
            let idx_ch: usize = ch as usize - 'a' as usize;
            ans.push((dict[idx_ch] + 'a' as u8) as char);
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let key: String = "the quick brown fox jumps over the lazy dog".to_owned();
        let message: String = "vkbs bs t suepuv".to_owned();
        let actual = Solution::decode_message(key, message);
        let expected = "this is a secret";
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_sample_input_2_should_return_expected() {
        let key: String = "eljuxhpwnyrdgtqkviszcfmabo".to_owned();
        let message: String = "zwx hnfx lqantp mnoeius ycgk vcnjrdb".to_owned();
        let actual = Solution::decode_message(key, message);
        let expected = "the five boxing wizards jump quickly";
        assert_eq!(expected, actual);
    }
}
