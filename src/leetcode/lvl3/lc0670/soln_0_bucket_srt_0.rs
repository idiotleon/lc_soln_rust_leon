/// @author: Leon
/// https://leetcode.com/problems/maximum-swap/
/// Time Complexity:    O(1)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/maximum-swap/discuss/107068/Java-simple-solution-O(n)-time
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut digits: Vec<u8> = Self::get_all_digits(num);
        let buckets: Vec<usize> = {
            let mut res: Vec<usize> = vec![0; 10];
            for (idx, &digit) in digits.iter().enumerate() {
                res[digit as usize] = idx;
            }
            res
        };
        let mut largest_digit: usize = 9;
        for (idx, &digit) in digits.iter().enumerate() {
            for k in (1 + digit as usize..=largest_digit).rev() {
                if buckets[k] > idx {
                    digits.swap(idx, buckets[k]);
                    return Self::construct_num(digits);
                }
            }
            largest_digit = digit as usize;
        }
        return num;
    }
    fn get_all_digits(num: i32) -> Vec<u8> {
        let mut digits: Vec<u8> = Vec::with_capacity(10);
        let mut num = num;
        while num > 0 {
            let digit = (num % 10) as u8;
            digits.push(digit);
            num /= 10;
        }
        digits.reverse();
        return digits;
    }
    fn construct_num(digits: Vec<u8>) -> i32 {
        let _len_ds: usize = digits.len();
        let mut res: i32 = 0;
        for digit in digits {
            res *= 10;
            res += digit as i32;
        }
        return res;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let num: i32 = 2736;
        let expected: i32 = 7236;
        let actual: i32 = Solution::maximum_swap(num);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let num: i32 = 9973;
        let expected: i32 = 9973;
        let actual: i32 = Solution::maximum_swap(num);
        assert_eq!(expected, actual);
    }
}
