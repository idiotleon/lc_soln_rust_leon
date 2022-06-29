/// @author: Leon
/// https://leetcode.com/problems/put-boxes-into-the-warehouse-i/
/// Time Complexity:    O(`_len_bs` * lg(`_len_bs`))
/// Space Complexity:   O(lg(`_len_bs`))
/// Reference:
/// https://leetcode.com/problems/put-boxes-into-the-warehouse-i/discuss/821966/C%2B%2B-Two-Approaches
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_boxes_in_warehouse(boxes: Vec<i32>, warehouse: Vec<i32>) -> i32 {
        let _len_bs: usize = boxes.len();
        let len_ws: usize = warehouse.len();
        let boxes: Vec<i32> = {
            let mut boxes = boxes;
            boxes.sort_by(|a, b| b.cmp(a));
            boxes
        };
        let mut cnt: usize = 0;
        for b in boxes {
            cnt += if cnt < len_ws && b <= warehouse[cnt] {
                1
            } else {
                0
            }
        }
        cnt as i32
    }
}
