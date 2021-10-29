/// https://leetcode.com/problems/sort-array-by-parity-ii/
///
/// Time Complexity:    O(`len_n`)
/// Space Complexity:   O(1)
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let len_n = nums.len();
        let mut nums = nums;
        let mut idx_even: usize = 0;
        let mut idx_odd: usize = 1;
        while idx_even < len_n && idx_odd < len_n {
            if nums[idx_even] % 2 == 0 {
                idx_even += 2;
            }
            if nums[idx_odd] % 2 == 1 {
                idx_odd += 2;
            }
            if idx_even < len_n && idx_odd < len_n {
                nums.swap(idx_even, idx_odd);
            }
        }
        nums
    }
}
