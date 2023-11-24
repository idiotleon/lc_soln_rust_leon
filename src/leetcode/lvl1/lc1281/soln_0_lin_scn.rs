/// @author: Leon
/// https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/
/// Time Complexity:    O(1)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let (product, sum): (i32, i32) = {
            let mut num = n;
            let mut sum: i32 = 0;
            let mut product: i32 = 1;
            while num > 0 {
                let digit = num % 10;
                sum += digit;
                product *= digit;
                num /= 10;
            }
            (product, sum)
        };
        return product - sum;
    }
}
