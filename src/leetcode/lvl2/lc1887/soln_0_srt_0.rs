/// @author: Leon
/// https://leetcode.com/problems/reduction-operations-to-make-the-array-elements-equal/
/// Time Complexity:    O(`len_ns` * lg(`len_ns`))
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/reduction-operations-to-make-the-array-elements-equal/?envType=daily-question&envId=2023-11-19
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
        let mut ans: i32 = 0;
        let mut up: i32 = 0;
        for idx in 1..len_ns {
            if sorted[idx - 1] != sorted[idx] {
                up += 1;
            }
            ans += up;
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
