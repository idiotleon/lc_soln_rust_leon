/// @author: Leon
/// https://leetcode.com/problems/find-the-k-beauty-of-a-number/
/// Time Complexity:    O(9) ~ O(1)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn divisor_substrings(num: i32, k: i32) -> i32 {
        let factor: i32 = 10_i32.pow(k as u32 - 1);
        let mut cnt: i32 = 0;
        let mut r_num: i32 = num / 10_i32.pow(k as u32 - 1);
        let mut k_num: i32 = num % 10_i32.pow(k as u32 - 1);
        while r_num > 0 {
            let digit = r_num % 10;
            k_num += factor * digit;
            if k_num != 0 && num % k_num == 0 {
                cnt += 1;
            }
            r_num /= 10;
            k_num /= 10;
        }
        cnt
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let num: i32 = 240;
        let k: i32 = 2;
        let expected = 2;
        let actual = Solution::divisor_substrings(num, k);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_sample_input_2_should_return_expected() {
        let num: i32 = 430043;
        let k: i32 = 2;
        let expected = 2;
        let actual = Solution::divisor_substrings(num, k);
        assert_eq!(expected, actual);
    }
}
