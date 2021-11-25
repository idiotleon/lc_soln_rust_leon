/// https://leetcode.com/problems/maximum-subarray/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(1)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let _len_n = nums.len();
        const RANGE: i32 = 1e4 as i32;
        let mut sum = 0;
        let mut largest = -(RANGE + 7);
        for num in nums {
            sum += num;
            if sum > largest {
                largest = sum;
            }
            if sum < 0 {
                sum = 0;
            }
        }
        largest
    }
}
