/// @author: Leon
/// https://leetcode.com/problems/count-largest-group/
/// Time Complexity:    O(`n` * avg_len_num)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let freqs: Vec<i32> = {
            let mut freqs: Vec<i32> = vec![0; 37];
            for num in 1..=n {
                let sum = Self::get_digit_sum(num);
                freqs[sum as usize] += 1;
            }
            freqs
        };
        let mut cnt: i32 = 0;
        let mut largest: i32 = -1;
        for freq in freqs {
            if freq > largest {
                cnt = 1;
                largest = freq;
            } else if freq == largest {
                cnt += 1;
            }
        }
        return cnt;
    }
    fn get_digit_sum(num: i32) -> i32 {
        let mut src = num;
        let mut sum: i32 = 0;
        while src > 0 {
            let digit = src % 10;
            sum += digit;
            src /= 10;
        }
        return sum;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let n: i32 = 13;
        let expected: i32 = 4;
        let actual: i32 = Solution::count_largest_group(n);
        assert_eq!(expected, actual);
    }
}
