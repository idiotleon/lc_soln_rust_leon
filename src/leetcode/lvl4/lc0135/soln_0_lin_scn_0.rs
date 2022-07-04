/// @author: Leon
/// https://leetcode.com/problems/candy/
/// Time Complexity:    O(`len_rs`)
/// Space Complexity:   O(1)
/// Reference:
/// https://leetcode.com/problems/candy/discuss/135698/Simple-solution-with-one-pass-using-O(1)-space
/// https://leetcode.com/problems/candy/discuss/2234434/C%2B%2B-oror-O(n)-Time-O(1)-Space-oror-Full-Explanation
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        if ratings.is_empty() {
            return 0;
        }
        let len_rs: usize = ratings.len();
        let mut cnt: i32 = 1;
        let mut up: i32 = 0;
        let mut down: i32 = 0;
        let mut peak: i32 = 0;
        for idx in 1..len_rs {
            if ratings[idx - 1] < ratings[idx] {
                up += 1;
                peak = up;
                down = 0;
                cnt += 1 + up;
            } else if ratings[idx - 1] == ratings[idx] {
                peak = 0;
                up = 0;
                down = 0;
                cnt += 1;
            } else {
                up = 0;
                down += 1;
                cnt += 1 + down + if peak >= down { -1 } else { 0 };
            }
        }
        cnt
    }
}
