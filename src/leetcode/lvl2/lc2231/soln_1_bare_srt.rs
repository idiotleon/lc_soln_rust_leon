struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_integer(num: i32) -> i32 {
        let digits: Vec<i32> = {
            let mut digits: Vec<i32> = Vec::new();
            let mut r_num: i32 = num;
            while r_num > 0 {
                digits.push(r_num % 10);
                r_num /= 10;
            }
            digits.reverse();
            digits
        };
        let (even, odd): (Vec<i32>, Vec<i32>) = {
            let (mut even, mut odd): (Vec<i32>, Vec<i32>) =
                digits.iter().partition(|&e| e % 2 == 0);
            even.sort();
            odd.sort();
            (even, odd)
        };
        let len_even: usize = even.len();
        let len_odd: usize = odd.len();
        let mut idx_even: usize = if len_even > 0 { len_even - 1 } else { 0 };
        let mut idx_odd: usize = if len_odd > 0 { len_odd - 1 } else { 0 };
        let mut ans: i32 = 0;
        for digit in digits {
            if digit % 2 == 0 {
                ans = ans * 10 + even[idx_even];
                if idx_even > 0 {
                    idx_even -= 1;
                }
            } else {
                ans = ans * 10 + odd[idx_odd];
                if idx_odd > 0 {
                    idx_odd -= 1;
                }
            };
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let num: i32 = 1234;
        let expected: i32 = 3412;
        let actual: i32 = Solution::largest_integer(num);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_sample_input_2_should_return_expected() {
        let num: i32 = 65875;
        let expected: i32 = 87655;
        let actual: i32 = Solution::largest_integer(num);
        assert_eq!(expected, actual);
    }
}
