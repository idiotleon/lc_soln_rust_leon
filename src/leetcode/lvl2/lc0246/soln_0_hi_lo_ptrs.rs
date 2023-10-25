use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/strobogrammatic-number/
/// Time Complexity:    O(`len_s`)
/// Space Complexity:   O(1) / O(`len_s`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_strobogrammatic(num: String) -> bool {
        let len_s: usize = num.len();
        let dict: HashMap<char, char> = {
            let mut dict: HashMap<char, char> = HashMap::with_capacity(5);
            dict.insert('0', '0');
            dict.insert('1', '1');
            dict.insert('6', '9');
            dict.insert('8', '8');
            dict.insert('9', '6');
            dict
        };
        let chs: Vec<char> = num.chars().collect();
        let mut lo: usize = 0;
        let mut hi: usize = len_s - 1;
        while lo <= hi {
            if let Some(&expected) = dict.get(&chs[lo]) {
                if expected != chs[hi] {
                    return false;
                }
            } else {
                return false;
            }
            lo += 1;
            hi -= 1;
        }
        return true;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let num: String = "69".to_owned();
        let expected: bool = true;
        let actual: bool = Solution::is_strobogrammatic(num);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_test_case_30() {
        let num: String = "2".to_owned();
        let expected: bool = false;
        let actual: bool = Solution::is_strobogrammatic(num);
        assert_eq!(expected, actual);
    }
}
