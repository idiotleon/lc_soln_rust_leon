/// @author: Leon
/// https://leetcode.com/problems/third-maximum-number/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(3) ~ O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let _len_n: usize = nums.len();
        const RANGE_LOWER: i64 = i64::MIN;
        let mut max1: Option<i64> = None;
        let mut max2: Option<i64> = None;
        let mut max3: Option<i64> = None;
        for num in nums {
            let num: i64 = num as i64;
            if num > max1.unwrap_or(RANGE_LOWER) {
                max3 = max2;
                max2 = max1;
                max1 = Some(num);
            } else if num != max1.unwrap_or(RANGE_LOWER) && num > max2.unwrap_or(RANGE_LOWER) {
                max3 = max2;
                max2 = Some(num);
            } else if num != max1.unwrap_or(RANGE_LOWER)
                && num != max2.unwrap_or(RANGE_LOWER)
                && num > max3.unwrap_or(RANGE_LOWER)
            {
                max3 = Some(num);
            }
        }
        if max3.is_none() {
            max1.unwrap() as i32
        } else {
            max3.unwrap() as i32
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_test_case_25() {
        let nums: Vec<i32> = vec![1, 2, 2, 5, 3, 5];
        let actual = Solution::third_max(nums);
        let expected = 2;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_test_case_27() {
        let nums: Vec<i32> = vec![1, 2, -2147483648];
        let actual = Solution::third_max(nums);
        let expected = -2147483648;
        assert_eq!(expected, actual);
    }
}
