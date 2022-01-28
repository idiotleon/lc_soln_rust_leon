/// @author: Leon
/// https://leetcode.com/problems/maximum-product-of-three-numbers/
/// Time Complexity:    O(`_len_n`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let _len_n: usize = nums.len();
        const RANGE: i32 = 1000 + 7;
        let mut max1: i32 = -RANGE;
        let mut max2: i32 = -RANGE;
        let mut max3: i32 = -RANGE;
        let mut min1: i32 = RANGE;
        let mut min2: i32 = RANGE;
        for num in nums {
            if num > max1 {
                max3 = max2;
                max2 = max1;
                max1 = num;
            } else if num > max2 {
                max3 = max2;
                max2 = num;
            } else if num > max3 {
                max3 = num;
            }
            if num < min1 {
                min2 = min1;
                min1 = num;
            } else if num < min2 {
                min2 = num;
            }
        }
        let product1 = max1 * max2 * max3;
        let product2 = max1 * min1 * min2;
        std::cmp::max(product1, product2)
    }
}