/// @author: Leon
/// https://leetcode.com/problems/replace-elements-with-greatest-element-on-right-side/
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn replace_elements(mut nums: Vec<i32>) -> Vec<i32> {
        let len_n: usize = nums.len();
        let mut max: i32 = -1;
        for idx in (0..len_n).rev() {
            let num = nums[idx];
            nums[idx] = max;
            max = std::cmp::max(max, num);
        }
        nums
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let nums = vec![17, 18, 5, 4, 6, 1];
        let actual = Solution::replace_elements(nums);
        let expected = vec![18, 6, 6, 6, 1, -1];
        assert_eq!(expected, actual);
    }
}
