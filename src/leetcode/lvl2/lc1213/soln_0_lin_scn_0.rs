/// @author: Leon
/// https://leetcode.com/problems/intersection-of-three-sorted-arrays/
/// Time Complexity:    O(min(`len_ns1`, `len_ns2`, `len_ns3`))
/// Space Complexity:   O(min(`len_ns1`, `len_ns2`, `len_ns3`)) / O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn arrays_intersection(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let len_ns1: usize = nums1.len();
        let len_ns2: usize = nums2.len();
        let len_ns3: usize = nums3.len();
        let mut idx1: usize = 0;
        let mut idx2: usize = 0;
        let mut idx3: usize = 0;
        let mut ans: Vec<i32> = Vec::with_capacity(len_ns1);
        while idx1 < len_ns1 && idx2 < len_ns2 && idx3 < len_ns3 {
            let num1: i32 = nums1[idx1];
            let num2: i32 = nums2[idx2];
            let num3: i32 = nums3[idx3];
            if num1 == num2 && num2 == num3 {
                ans.push(num1);
                idx1 += 1;
                idx2 += 1;
                idx3 += 1;
            } else {
                if num1 <= num2 && num1 <= num3 {
                    idx1 += 1;
                } else if num2 <= num1 && num2 <= num3 {
                    idx2 += 1;
                } else {
                    idx3 += 1;
                }
            }
        }
        return ans;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let nums1: Vec<i32> = vec![1, 2, 3, 4, 5];
        let nums2: Vec<i32> = vec![1, 2, 5, 7, 9];
        let nums3: Vec<i32> = vec![1, 3, 4, 5, 8];
        let expected: Vec<i32> = vec![1, 5];
        let actual: Vec<i32> = Solution::arrays_intersection(nums1, nums2, nums3);
        assert_eq!(expected, actual);
    }
}
