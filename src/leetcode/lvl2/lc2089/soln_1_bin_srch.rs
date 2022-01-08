/// @author: Leon
/// https://leetcode.com/problems/find-target-indices-after-sorting-array/
/// Time Complexity:    O(`_len_n` * lg(`_len_n`)) + O(2 * lg(`_len_n`)) ~ O(`_len_n` * lg(`_len_n`))
/// Space Complexity:   O(1) / O(`_len_n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn target_indices(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let _len_n: usize = nums.len();
        let sorted: Vec<i32> = {
            let mut sorted: Vec<i32> = nums;
            sorted.sort();
            sorted
        };
        let lo: usize = Self::bs_lower_bound(&sorted, target);
        let hi: isize = Self::bs_higher_bound(&sorted, target);
        if hi < 0 {
            return vec![];
        }
        let hi: usize = hi as usize;
        let ans: Vec<i32> = {
            let mut ans: Vec<i32> = Vec::with_capacity(hi - lo + 1);
            for idx in lo..=hi {
                ans.push(idx as i32);
            }
            ans
        };
        ans
    }
    fn bs_lower_bound(nums: &Vec<i32>, target: i32) -> usize {
        let len_n: usize = nums.len();
        let mut lo: usize = 0;
        let mut hi: usize = len_n;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] >= target {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        lo
    }
    fn bs_higher_bound(nums: &Vec<i32>, target: i32) -> isize {
        let len_n: usize = nums.len();
        let mut lo: usize = 0;
        let mut hi: usize = len_n;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] <= target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        hi as isize - 1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let nums: Vec<i32> = vec![1, 2, 5, 2, 3];
        let target = 2;
        let actual = Solution::target_indices(nums, target);
        let expected = vec![1, 2];
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_test_case_47() {
        let nums: Vec<i32> = vec![
            19, 35, 87, 45, 93, 10, 79, 41, 57, 75, 66, 56, 74, 25, 59, 71, 19, 18, 84, 28, 32, 63,
            73, 97, 53,
        ];
        let target = 8;
        let actual = Solution::target_indices(nums, target);
        let expected: Vec<i32> = Vec::new();
        assert_eq!(expected, actual);
    }
}
