/// @author: Leon
/// https://leetcode.com/problems/rearrange-spaces-between-words/
/// Time Complexity:    O(`len_t`)
/// Space Complexity:   O(`len_t`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let _len_t: usize = text.len();
        const SPACE: char = ' ';
        let mut words: Vec<String> = Vec::new();
        let cnt_spcs: usize = {
            let mut cnt_spcs: usize = 0;
            let mut word: String = "".to_owned();
            for ch in text.chars() {
                match ch {
                    SPACE => {
                        cnt_spcs += 1;
                        if !word.is_empty() {
                            words.push(word.to_owned());
                        }
                        word.clear();
                    }
                    _ => word.push(ch),
                }
            }
            if !word.is_empty() {
                words.push(word.to_owned());
            }
            cnt_spcs
        };
        let mut ans: String = "".to_owned();
        let len_wrds = words.len();
        if len_wrds == 1 {
            ans.push_str(&words[0]);
            for _ in 0..cnt_spcs {
                ans.push(SPACE);
            }
            return ans;
        }
        let step = cnt_spcs / (len_wrds - 1);
        for (idx, word) in words.into_iter().enumerate() {
            ans.push_str(&word);
            if idx == len_wrds - 1 {
                break;
            }
            for _ in 0..step {
                ans.push(SPACE);
            }
        }
        // to append spaces left/modularized
        let mod_wrds = cnt_spcs % (len_wrds - 1);
        for _ in 0..mod_wrds {
            ans.push(SPACE);
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let text = "  this   is  a sentence ".to_owned();
        let actual = Solution::reorder_spaces(text);
        let expected = "this   is   a   sentence".to_owned();
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let text = " practice   makes   perfect".to_owned();
        let actual = Solution::reorder_spaces(text);
        let expected = "practice   makes   perfect ".to_owned();
        assert_eq!(expected, actual);
    }
}
