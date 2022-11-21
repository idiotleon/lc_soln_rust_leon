/// @author: Leon
/// https://leetcode.com/problems/find-the-distance-value-between-two-arrays/description/
/// Time Complexity:    O(`_len_ns1` * `len_ns2`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_the_distance_value(nums1: Vec<i32>, nums2: Vec<i32>, d: i32) -> i32 {
        let _len_ns1: usize = nums1.len();
        let len_ns2: usize = nums2.len();
        let mut cnt: i32 = 0;
        for &num1 in &nums1 {
            for (idx, &num2) in nums2.iter().enumerate() {
                if (num1 - num2).abs() <= d {
                    break;
                }
                if idx == len_ns2 - 1 {
                    cnt += 1;
                }
            }
        }
        return cnt;
    }
}
