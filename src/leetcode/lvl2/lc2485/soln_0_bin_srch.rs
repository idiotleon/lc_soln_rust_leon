/// @author: Leon
/// https://leetcode.com/problems/find-the-pivot-integer/
/// Time Complexity:    O(lg(`n`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn pivot_integer(n: i32) -> i32 {
        let mut lo: i32 = 1;
        let mut hi: i32 = n;
        while lo <= hi {
            let mid: i32 = lo + (hi - lo) / 2;
            let sum_lo: i32 = Self::get_sum(1, mid);
            let sum_hi: i32 = Self::get_sum(mid, n);
            if sum_lo == sum_hi {
                return mid;
            }
            if sum_lo > sum_hi {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }
        return -1;
    }
    fn get_sum(lo: i32, hi: i32) -> i32 {
        return (hi - lo + 1) * (lo + hi) / 2;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let n: i32 = 8;
        let actual: i32 = Solution::pivot_integer(n);
        let expected: i32 = 6;
        assert_eq!(expected, actual);
    }
}
