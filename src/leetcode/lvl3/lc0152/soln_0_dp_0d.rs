/// https://leetcode.com/problems/maximum-product-subarray/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference
/// https://leetcode.com/problems/maximum-product-subarray/discuss/48230/Possibly-simplest-solution-with-O(n)-time-complexity
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut prev_min = 1;
        let mut prev_max = 1;
        let mut max_product = i32::MIN;
        for num in nums {
            let cur_min = *vec![num, prev_min * num, prev_max * num]
                .iter()
                .min()
                .unwrap();
            let cur_max = *vec![num, prev_min * num, prev_max * num]
                .iter()
                .max()
                .unwrap();
            max_product = std::cmp::max(max_product, cur_max);
            prev_min = cur_min;
            prev_max = cur_max;
        }
        max_product
    }
}
