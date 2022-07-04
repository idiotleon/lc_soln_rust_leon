/// @author: Leon
/// https://leetcode.com/problems/candy/
/// Time Complexity:    O(`len_rs`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/candy/discuss/2234434/C%2B%2B-oror-O(n)-Time-O(1)-Space-oror-Full-Explanation
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let len_rs: usize = ratings.len();
        let mut cnt_candy: i32 = len_rs as i32;
        let mut idx: usize = 1;
        while idx < len_rs {
            if ratings[idx - 1] == ratings[idx] {
                idx += 1;
                continue;
            }
            // as the increasing slope
            let mut up: i32 = 0;
            while ratings[idx - 1] < ratings[idx] {
                up += 1;
                cnt_candy += up;
                idx += 1;
                if idx == len_rs {
                    return cnt_candy as i32;
                }
            }
            // as the decreasing slope
            let mut down: i32 = 0;
            while idx < len_rs && ratings[idx - 1] > ratings[idx] {
                down += 1;
                cnt_candy += down;
                idx += 1;
            }
            // to keep only the higher peak
            cnt_candy -= std::cmp::min(up, down);
        }
        cnt_candy as i32
    }
}
