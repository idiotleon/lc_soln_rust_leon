/// @author: Leon
/// https://leetcode.com/problems/reduction-operations-to-make-the-array-elements-equal/
/// Time Complexity:    O(`len_ns` * lg(`len_ns`))
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reduction_operations(nums: Vec<i32>) -> i32 {
        let len_ns: usize = nums.len();
        let sorted: Vec<i32> = {
            let mut nums = nums;
            nums.sort();
            nums
        };
        let lo: isize = 0;
        let mut hi: isize = len_ns as isize - 1;
        if sorted[lo as usize] == sorted[hi as usize] {
            return 0;
        }
        let mut ans: i32 = 0;
        while lo < hi {
            while lo < hi && sorted[hi as usize - 1] == sorted[hi as usize] {
                hi -= 1;
            }
            ans += len_ns as i32 - hi as i32;
            if sorted[hi as usize - 1] == sorted[0] {
                break;
            }
            hi -= 1;
        }
        return ans;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let nums: Vec<i32> = vec![5, 1, 3];
        let expected: i32 = 3;
        let actual: i32 = Solution::reduction_operations(nums);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_2() {
        let nums: Vec<i32> = vec![1, 1, 1];
        let expected: i32 = 0;
        let actual: i32 = Solution::reduction_operations(nums);
        assert_eq!(expected, actual);
    }
    #[test]
    fn it_works_with_sample_input_3() {
        let nums: Vec<i32> = vec![1, 1, 2, 2, 3];
        let expected: i32 = 4;
        let actual: i32 = Solution::reduction_operations(nums);
        assert_eq!(expected, actual);
    }
}
