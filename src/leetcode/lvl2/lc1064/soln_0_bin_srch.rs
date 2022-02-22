/// @author: Leon
/// https://leetcode.com/problems/fixed-point/
/// Time Complexity:    O(lg(`len_n`))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/fixed-point/discuss/303401/JavaC++Python-Binary-Search-0-in-Ai-i/283505
/// https://leetcode.com/problems/fixed-point/discuss/303401/JavaC%2B%2BPython-Binary-Search-0-in-Ai-i
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn fixed_point(nums: Vec<i32>) -> i32 {
        let len_n: usize = nums.len();
        let mut lo: usize = 0;
        let mut hi: usize = len_n;
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if nums[mid] >= mid as i32 {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        if lo < len_n && nums[lo] == lo as i32 {
            lo as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let nums: Vec<i32> = vec![-10, -5, 0, 3, 7];
        let actual = Solution::fixed_point(nums);
        let expected = 3;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_test_case_45() {
        let nums: Vec<i32> = vec![0, 1, 3, 7, 8, 9];
        let actual = Solution::fixed_point(nums);
        let expected = 0;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_test_case_46() {
        let nums: Vec<i32> = vec![-10, -5, -2, 0, 4, 5, 6, 7, 8, 9, 10];
        let actual = Solution::fixed_point(nums);
        let expected = 4;
        assert_eq!(expected, actual);
    }
}
