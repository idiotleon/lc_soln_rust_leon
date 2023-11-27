/// @author: Leon
/// https://leetcode.com/problems/3sum-smaller/
/// Time Complexity:    O(`len_ns` ^ 2)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn three_sum_smaller(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let len_ns: usize = nums.len();
        let sorted: Vec<i32> = {
            let mut nums = nums;
            nums.sort();
            nums
        };
        let mut ans: i32 = 0;
        for lo in 0..len_ns {
            let mut mid = lo + 1;
            let mut hi = len_ns - 1;
            while mid < hi {
                let sum = sorted[lo] + sorted[mid] + sorted[hi];
                if sum < target {
                    ans += hi as i32 - mid as i32;
                    mid += 1;
                } else {
                    hi -= 1;
                }
            }
        }
        return ans;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let nums: Vec<i32> = vec![-2, 0, 1, 3];
        let target = 2;
        let expected: i32 = 2;
        let actual: i32 = Solution::three_sum_smaller(nums, target);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let nums: Vec<i32> = vec![];
        let target = 0;
        let expected: i32 = 0;
        let actual: i32 = Solution::three_sum_smaller(nums, target);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_3() {
        let nums: Vec<i32> = vec![0];
        let target = 0;
        let expected: i32 = 0;
        let actual: i32 = Solution::three_sum_smaller(nums, target);
        assert_eq!(expected, actual);
    }
}
