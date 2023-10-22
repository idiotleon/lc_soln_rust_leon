use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/minimum-sum-of-mountain-triplets-ii/description/
/// This is not yet a correct solution.
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        let sum_total: i32 = nums.iter().sum();
        let len_ns: usize = nums.len();
        let mut stk: VecDeque<usize> = VecDeque::with_capacity(len_ns);
        let mut ans = sum_total + 1;
        for idx in 0..len_ns {
            while let Some(&top) = stk.back() {
                if nums[idx] <= nums[top] {
                    if let Some(top) = stk.pop_back() {
                        if !stk.is_empty() {
                            let first = nums[*stk.front().unwrap()];
                            if first < nums[top] && nums[top] > nums[idx] {
                                let sum: i32 = first + nums[top] + nums[idx];
                                ans = std::cmp::min(ans, sum);
                            }
                        }
                    }
                } else {
                    break;
                }
            }
            stk.push_back(idx);
        }
        return if ans == sum_total + 1 { -1 } else { ans };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let nums = vec![8, 6, 1, 5, 3];
        let expected = 9;
        let actual = Solution::minimum_sum(nums);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let nums = vec![5, 4, 8, 7, 10, 2];
        let expected = 13;
        let actual = Solution::minimum_sum(nums);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_3() {
        let nums = vec![6, 5, 4, 3, 4, 5];
        let expected = -1;
        let actual = Solution::minimum_sum(nums);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_test_case_0() {
        let nums = vec![1, 2, 1, 2];
        let expected = 4;
        let actual = Solution::minimum_sum(nums);
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_works_with_test_case_1() {
        let nums = vec![2, 2, 1];
        let expected = -1;
        let actual = Solution::minimum_sum(nums);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_test_case_2() {
        let nums = vec![1, 2, 2];
        let expected = -1;
        let actual = Solution::minimum_sum(nums);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_test_case_3() {
        let nums = vec![2, 3, 2, 1];
        let expected = 6;
        let actual = Solution::minimum_sum(nums);
        assert_eq!(expected, actual);
    }
}
