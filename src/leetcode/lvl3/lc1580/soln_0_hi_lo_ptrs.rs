/// @author: Leon
/// https://leetcode.com/problems/put-boxes-into-the-warehouse-ii/
/// Time Complexity:    O(`len_bs` * lg(`len_bs`)) + O(`len_bs`) ~ O(`len_bs` * lg(`len_bs`))
/// Space Complexity:   O(lg(`len_bs`))
/// Reference:
/// https://leetcode.com/problems/put-boxes-into-the-warehouse-ii/discuss/839298/JAVA-Short-O(NlgN%2BM)-Time-O(1)-Space
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
        let mut cnt: i32 = 0;
        let mut lo: usize = 0;
        let mut hi: usize = len_ws - 1;
        for idx in (0..len_bs).rev() {
            if lo > hi {
                break;
            }
            if boxes[idx] <= warehouse[lo] {
                lo += 1;
                cnt += 1;
            } else if boxes[idx] <= warehouse[hi] {
                hi -= 1;
                cnt += 1;
            }
        }
        cnt
    }
}
