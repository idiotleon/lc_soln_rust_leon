/// @author: Leon
/// https://leetcode.com/problems/remove-duplicates-from-sorted-array/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len_n: usize = nums.len();
        if len_n < 2 {
            return len_n as i32;
        }
        let mut lo: usize = 0;
        let mut hi: usize = 1;
        while hi < len_n {
            if nums[hi - 1] == nums[hi] {
                hi += 1;
            } else {
                lo += 1;
                nums[lo] = nums[hi];
                hi += 1;
            }
        }
        lo as i32 + 1
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let mut nums: Vec<i32> = vec![1, 1, 2];
        let actual = Solution::remove_duplicates(&mut nums);
        let expected = 2;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let mut nums: Vec<i32> = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let actual = Solution::remove_duplicates(&mut nums);
        let expected = 5;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_custom_input_1() {
        let mut nums: Vec<i32> = vec![1, 1];
        let actual = Solution::remove_duplicates(&mut nums);
        let expected = 1;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_custom_input_2() {
        let mut nums: Vec<i32> = vec![1, 1, 1, 1, 1, 2];
        let actual = Solution::remove_duplicates(&mut nums);
        let expected = 2;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_custom_input_3() {
        let mut nums: Vec<i32> = vec![1, 2, 3];
        let actual = Solution::remove_duplicates(&mut nums);
        let expected = 3;
        assert_eq!(expected, actual);
    }
}
