/// https://leetcode.com/problems/put-boxes-into-the-warehouse-i/
/// 
/// Time Complexity:    O()
/// Space Complexity:   O()
/// 
/// Reference:
/// https://leetcode.com/problems/put-boxes-into-the-warehouse-i/discuss/821966/C%2B%2B-Two-Approaches
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_boxes_in_warehouse(boxes: Vec<i32>, warehouse: Vec<i32>) -> i32 {
        let len_wh = warehouse.len();
        
        let mut count: usize = 0;
        
        let mut boxes = boxes;
        boxes.sort_by(|a, b| b.partial_cmp(a).unwrap());
        
        for &b in boxes.iter(){
            if count < len_wh && b <= warehouse[count]{
                count += 1;
            }
        }
        
        count as i32
    }
}