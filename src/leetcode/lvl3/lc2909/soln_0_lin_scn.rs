/// @author: Leon
/// https://leetcode.com/problems/minimum-sum-of-mountain-triplets-ii/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(`len_ns`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_sum(nums: Vec<i32>) -> i32 {
        const IMPL: i32 = -1;
        const RANGE: i32 = 10e8 as i32 + 7;
        let len_ns: usize = nums.len();
        let lo_mins: Vec<i32> = {
            let mut mins = vec![RANGE; len_ns];
            let mut min = RANGE;
            for idx in 0..len_ns {
                min = std::cmp::min(min, nums[idx]);
                mins[idx] = min;
            }
            mins
        };
        let hi_mins: Vec<i32> = {
            let mut mins = vec![RANGE; len_ns];
            let mut min = RANGE;
            for idx in (0..len_ns).rev() {
                min = std::cmp::min(min, nums[idx]);
                mins[idx] = min;
            }
            mins
        };
        let mut ans: i32 = IMPL;
        for idx in 0..len_ns {
            if nums[idx] > lo_mins[idx] && nums[idx] > hi_mins[idx] {
                let sum = lo_mins[idx] + nums[idx] + hi_mins[idx];
                if ans == IMPL {
                    ans = sum;
                } else {
                    ans = std::cmp::min(ans, sum);
                }
            }
        }
        return ans;
    }
}

#[cfg(test)]
mod test {
    use super::Solution;
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
    fn it_does_not_works_with_sample_input_3() {
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
    fn it_does_not_work_with_test_case_1() {
        let nums = vec![2, 2, 1];
        let expected = -1;
        let actual = Solution::minimum_sum(nums);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_does_not_work_with_test_case_2() {
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
    #[test]
    fn it_works_with_test_case_752() {
        let nums = vec![99999999, 100000000, 99999999];
        let expected = 299999998;
        let actual = Solution::minimum_sum(nums);
        assert_eq!(expected, actual);
    }
}
