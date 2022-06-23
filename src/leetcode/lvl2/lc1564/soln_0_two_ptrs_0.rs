use std::cmp::min;

/// @author: Leon
/// https://leetcode.com/problems/put-boxes-into-the-warehouse-i/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/put-boxes-into-the-warehouse-i/discuss/821966/C%2B%2B-Two-Approaches
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_boxes_in_warehouse(boxes: Vec<i32>, warehouse: Vec<i32>) -> i32 {
        let len_wh = warehouse.len();
        let len_b = boxes.len();
        let mut count: usize = 0;
        let mut boxes = boxes;
        boxes.sort();
        let mut warehouse = warehouse;
        for i in 1..len_wh {
            warehouse[i] = min(warehouse[i], warehouse[i - 1]);
        }
        for i in (0..len_wh).rev() {
            if count < len_b && boxes[count] <= warehouse[i] {
                count += 1;
            }
        }
        count as i32
    }
}
