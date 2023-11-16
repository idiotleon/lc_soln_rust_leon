use std::collections::HashSet;

/// @author: Leon
/// https://leetcode.com/problems/find-unique-binary-string/
/// Time Complexity:    O(`len_s`) ~ O(1)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let _len_ns: usize = nums.len();
        let len_s: usize = nums[0].len();
        let mut set: HashSet<u32> = nums.into_iter().map(|num| Self::get_bitmask(num)).collect();
        for num in 0..(1 << len_s) {
            if set.insert(num) {
                return Self::convert(len_s, num);
            }
        }
        unreachable!()
    }
    fn get_bitmask(num: String) -> u32 {
        let len_s: usize = num.len();
        let mut res: u32 = 0;
        for (idx, ch) in num.chars().into_iter().enumerate() {
            if ch == '1' {
                res |= 1 << (len_s - idx - 1);
            }
        }
        return res;
    }
    fn convert(len_s: usize, num: u32) -> String {
        let mut res: Vec<char> = vec!['0'; len_s];
        let mut idx: usize = 0;
        let mut num = num;
        while num > 0 {
            let digit = num % 2;
            if digit == 1 {
                res[len_s - 1 - idx] = '1';
            }
            num /= 2;
            idx += 1;
        }
        return res.into_iter().collect();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;
    #[test]
    fn it_works_with_sample_input_1() {
        let nums: Vec<String> = vec!["01".to_owned(), "10".to_owned()];
        let expected: HashSet<String> =
            vec!["00".to_owned(), "11".to_owned()].into_iter().collect();
        let actual: String = Solution::find_different_binary_string(nums);
        assert!(expected.contains(&actual));
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let nums: Vec<String> = vec!["00".to_owned(), "01".to_owned()];
        let expected: HashSet<String> =
            vec!["10".to_owned(), "11".to_owned()].into_iter().collect();
        let actual: String = Solution::find_different_binary_string(nums);
        assert!(expected.contains(&actual));
    }
    #[test]
    fn it_works_with_test_case_164() {
        let nums: Vec<String> = vec![
            "0000000000000111".to_owned(),
            "0000000000000100".to_owned(),
            "0000000000001001".to_owned(),
            "0000000000000011".to_owned(),
            "0000000000001000".to_owned(),
            "1111111111111111".to_owned(),
            "0000000000000000".to_owned(),
            "0000000000000010".to_owned(),
            "0000000000001010".to_owned(),
            "0000000000000110".to_owned(),
            "0000000000001011".to_owned(),
            "0000000000001111".to_owned(),
            "0000000000001110".to_owned(),
            "0000000000000101".to_owned(),
            "0000000000001101".to_owned(),
            "0000000000001100".to_owned(),
        ];
        // this is not exhaustive
        let expected: HashSet<String> = vec!["0000000000000001".to_owned()].into_iter().collect();
        let actual: String = Solution::find_different_binary_string(nums);
        assert!(expected.contains(&actual));
    }
}
