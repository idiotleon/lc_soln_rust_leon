use std::collections::HashSet;

/// https://leetcode.com/problems/two-out-of-three/
/// Time Complexity:    O(`_len_n1` + `_len_n2` + `_len_n3`)
/// Space Complexity:   O(`_len_n1` + `_len_n2` + `_len_n3`)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    const RANGE: usize = 100 + 1;
    pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let _len_n1 = nums1.len();
        let _len_n2 = nums2.len();
        let _len_n3 = nums3.len();
        let mut vec_num_to_set: Vec<HashSet<i32>> = vec![HashSet::new(); Self::RANGE];
        for num in nums1 {
            vec_num_to_set[num as usize].insert(1);
        }
        for num in nums2 {
            vec_num_to_set[num as usize].insert(2);
        }
        for num in nums3 {
            vec_num_to_set[num as usize].insert(3);
        }
        let ans: Vec<i32> = {
            let mut ans: Vec<i32> = Vec::new();
            for (idx, hs) in vec_num_to_set.into_iter().enumerate() {
                if hs.len() >= 2 {
                    ans.push(idx as i32);
                }
            }
            ans
        };
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sample_input_1() {
        let nums1 = vec![1, 1, 3, 2];
        let nums2 = vec![2, 3];
        let nums3 = vec![3];
        let actual = Solution::two_out_of_three(nums1, nums2, nums3);
        let expected = vec![2, 3];
        assert_eq!(actual, expected);
    }
}
