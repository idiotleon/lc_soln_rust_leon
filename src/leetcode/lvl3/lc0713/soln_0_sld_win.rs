/// @author: Leon
/// https://leetcode.com/problems/subarray-product-less-than-k/
/// Time Complexity:    O(`len_ns`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        let len_ns: usize = nums.len();
        let mut product: i32 = 1;
        let mut lo: usize = 0;
        let mut hi: usize = 0;
        let mut cnt: i32 = 0;
        while hi < len_ns {
            product *= nums[hi];
            while lo <= hi && product >= k {
                product /= nums[lo];
                lo += 1;
            }
            cnt += (hi - lo + 1) as i32;
            hi += 1;
        }
        cnt
    }
}
