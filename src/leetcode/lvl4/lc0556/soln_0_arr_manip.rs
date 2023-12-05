/// @author: Leon
/// https://leetcode.com/problems/next-greater-element-iii/
/// Time Complexity:    O(`n`)
/// Space Complexity:   O(`n`)
/// Reference:
/// https://leetcode.com/problems/next-greater-element-iii/discuss/101824/Simple-Java-solution-(4ms)-with-explanation./105491
/// https://leetcode.com/problems/next-greater-element-iii/discuss/101824/Simple-Java-solution-(4ms)-with-explanation.
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut digits = Self::get_digits(n);
        let len_ds: usize = digits.len();
        let mut lo: isize = len_ds as isize - 2;
        while lo >= 0 && digits[lo as usize] >= digits[lo as usize + 1] {
            lo -= 1;
        }
        if lo < 0 {
            return -1;
        }
        let lo: usize = lo as usize;
        let mut hi: usize = len_ds as usize - 1;
        while digits[hi] <= digits[lo] {
            hi -= 1;
        }
        digits.swap(lo, hi);
        Self::reverse_since(lo + 1, &mut digits);
        return Self::convert(digits);
    }
    fn get_digits(n: i32) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::with_capacity(32);
        let mut n = n;
        while n > 0 {
            let digit = (n % 10) as u8;
            res.push(digit);
            n /= 10;
        }
        res.reverse();
        return res;
    }
    fn convert(digits: Vec<u8>) -> i32 {
        let mut res: i64 = 0;
        for digit in digits {
            res *= 10;
            res += digit as i64;
        }
        // to prevent i32 overflow
        if res > i32::MAX as i64 {
            return -1;
        }
        return res as i32;
    }
    fn reverse_since(start: usize, digits: &mut Vec<u8>) {
        let len_ns: usize = digits.len();
        let mut lo: usize = start;
        let mut hi: usize = len_ns - 1;
        while lo < hi {
            digits.swap(lo, hi);
            lo += 1;
            hi -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let n: i32 = 12;
        let expected: i32 = 21;
        let actual: i32 = Solution::next_greater_element(n);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let n: i32 = 21;
        let expected: i32 = -1;
        let actual: i32 = Solution::next_greater_element(n);
        assert_eq!(expected, actual);
    }
}
