/// @author: Leon
/// https://leetcode.com/problems/candy/
/// Time Complexity:    O(`len_rs`)
/// Space Complexity:   O(`len_rs`)
/// Reference:
/// https://leetcode.com/problems/candy/discuss/42769/A-simple-solution/336970
/// https://leetcode.com/problems/candy/discuss/42769/A-simple-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let len_rs: usize = ratings.len();
        let dist: Vec<i32> = {
            let mut dist: Vec<i32> = vec![1; len_rs];
            for idx in 1..len_rs {
                if ratings[idx - 1] < ratings[idx] {
                    dist[idx] = 1 + dist[idx - 1];
                }
            }
            for idx in (1..len_rs).rev() {
                if ratings[idx - 1] > ratings[idx] {
                    dist[idx - 1] = std::cmp::max(1 + dist[idx], dist[idx - 1]);
                }
            }
            dist
        };
        dist.into_iter().sum::<i32>()
    }
}
