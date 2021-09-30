/// https://leetcode.com/problems/put-boxes-into-the-warehouse-ii/
///
/// Time Complexity:    O(`len_bx` * lg(`len_bx`)) + O(`len_bx`) ~
/// Space Complexity:   O(1)
///
/// a greedy approach
///
/// Reference:
/// https://leetcode.com/problems/put-boxes-into-the-warehouse-ii/discuss/839298/JAVA-Short-O(NlgN%2BM)-Time-O(1)-Space
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_boxes_in_warehouse(boxes: Vec<i32>, warehouse: Vec<i32>) -> i32 {
        let len_wh: usize = warehouse.len();
        let mut lo: usize = 0;
        let mut hi: usize = len_wh - 1;

        let len_bx: usize = boxes.len();

        let mut boxes = boxes;
        boxes.sort();

        let mut count: usize = 0;

        for i in (0..len_bx).rev() {
            if lo > hi {
                break;
            }

            if boxes[i] <= warehouse[lo] {
                lo += 1;
                count += 1;
            } else if boxes[i] <= warehouse[hi] {
                hi -= 1;
                count += 1;
            }
        }
        count as i32
    }
}
