/// @author: Leon
/// https://leetcode.com/problems/smallest-value-of-the-rearranged-number/
/// Time Complexity:    O(15) ~ O(1)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn smallest_number(num: i64) -> i64 {
        let mut freqs: Vec<u8> = Self::get_freqs(num);
        let mut ans: i64 = 0;
        // to construct the number
        if num > 0 {
            for idx in 1..10 {
                while freqs[idx] > 0 {
                    while ans > 0 && freqs[0] > 0 {
                        ans *= 10;
                        freqs[0] -= 1;
                    }
                    ans *= 10;
                    ans += idx as i64;
                    freqs[idx] -= 1;
                }
            }
            // to add all left 0s as trailing 0s
            while ans > 0 && freqs[0] > 0 {
                ans *= 10;
                freqs[0] -= 1;
            }
        } else {
            for idx in (0..10).rev() {
                while freqs[idx] > 0 {
                    ans *= 10;
                    ans += idx as i64;
                    freqs[idx] -= 1;
                }
            }
            ans *= -1;
        }
        ans
    }
    // to get the frequencies of all the digits
    fn get_freqs(mut num: i64) -> Vec<u8> {
        if num < 0 {
            num = -num;
        }
        let mut freqs: Vec<u8> = vec![0; 10];
        while num != 0 {
            let digit = num % 10;
            freqs[digit as usize] += 1;
            num /= 10;
        }
        freqs
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn it_works_with_sample_input_1() {
        let num: i64 = 310;
        let actual: i64 = Solution::smallest_number(num);
        let expected: i64 = 103;
        assert_eq!(expected, actual);
    }
    #[test]
    pub fn it_works_with_sample_input_2() {
        let num: i64 = -7605;
        let actual: i64 = Solution::smallest_number(num);
        let expected: i64 = -7650;
        assert_eq!(expected, actual);
    }
    #[test]
    pub fn it_works_with_test_case_45() {
        let num: i64 = 1525;
        let actual: i64 = Solution::smallest_number(num);
        let expected: i64 = 1255;
        assert_eq!(expected, actual);
    }
}
