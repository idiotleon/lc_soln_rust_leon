/// @author: Leon
/// https://leetcode.com/problems/put-boxes-into-the-warehouse-i/
/// Time Complexity:    O(max(`len_bs`, `len_ws`) * lg(max(`len_bs`, `len_ws`)))
/// Space Complexity:   O(lg(max(`len_bs`, `len_ws`)))
/// Reference:
/// https://leetcode.com/problems/put-boxes-into-the-warehouse-i/discuss/821966/C%2B%2B-Two-Approaches
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_boxes_in_warehouse(boxes: Vec<i32>, warehouse: Vec<i32>) -> i32 {
        let len_bs: usize = boxes.len();
        let len_ws: usize = warehouse.len();
        let boxes: Vec<i32> = {
            let mut boxes = boxes;
            boxes.sort();
            boxes
        };
        let wh: Vec<i32> = {
            let mut wh = warehouse;
            for idx in 1..len_ws {
                wh[idx] = std::cmp::min(wh[idx - 1], wh[idx]);
            }
            wh
        };
        let mut cnt: usize = 0;
        for idx in (0..len_ws).rev() {
            cnt += if cnt < len_bs && boxes[cnt] <= wh[idx] {
                1
            } else {
                0
            }
        }
        cnt as i32
    }
}
