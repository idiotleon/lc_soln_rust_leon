/// @author: Leon
/// https://leetcode.com/problems/maximum-product-difference-between-two-pairs/
/// Time Complexity:    O(`_len_ns`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_product_difference(nums: Vec<i32>) -> i32 {
        let _len_ns = nums.len();
        const MAX: i32 = 1e4 as i32;
        let mut min = MAX;
        let mut sec_min = MAX;
        let mut max = 1;
        let mut sec_max = 1;
        for num in nums {
            if min > num {
                sec_min = min;
                min = num;
            } else if sec_min > num {
                sec_min = num;
            }
            if max < num {
                sec_max = max;
                max = num;
            } else if sec_max < num {
                sec_max = num;
            }
        }
        max * sec_max - min * sec_min
    }
}
