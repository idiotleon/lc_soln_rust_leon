/// @author: Leon
/// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/
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
        let mut lo: usize = 1;
        let mut hi: usize = 1;
        let mut cnt: u16 = 1;
        while hi < len_n {
            if nums[hi - 1] == nums[hi] {
                if cnt == 2 {
                    hi += 1;
                    continue;
                } else {
                    cnt += 1;
                    nums[lo] = nums[hi];
                    lo += 1;
                    hi += 1;
                }
            } else {
                cnt = 1;
                nums[lo] = nums[hi];
                lo += 1;
                hi += 1;
            }
        }
        lo as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let mut nums: Vec<i32> = vec![1, 1, 1, 2, 2, 3];
        let actual = Solution::remove_duplicates(&mut nums);
        let expected = 5;
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let mut nums: Vec<i32> = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let actual = Solution::remove_duplicates(&mut nums);
        let expected = 7;
        assert_eq!(expected, actual);
    }
}
