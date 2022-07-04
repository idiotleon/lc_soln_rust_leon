/// @author: Leon
/// https://leetcode.com/problems/candy/
/// Time Complexity:    O(`len_rs`)
/// Space Complexity:   O(`len_rs`)
/// Reference:
/// https://leetcode.com/problems/candy/discuss/42769/A-simple-solution/336970
/// https://leetcode.com/problems/candy/discuss/42769/A-simple-solution
/// https://leetcode.com/problems/candy/discuss/42794/Simple-O(n)-Java-solution-with-comments
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let len_rs: usize = ratings.len();
        let dist: Vec<i32> = {
            // the distribution of candies
            let mut dist: Vec<i32> = vec![1; len_rs];
            // to iterate over the `ratings` from left to right,
            // to maek sure the right that is more highly rated gets 1 more candy than the left
            for idx in 1..len_rs {
                if ratings[idx - 1] < ratings[idx] {
                    dist[idx] = 1 + dist[idx - 1];
                }
            }
            // to iterate over the `ratings` from right to left,
            // to make sure the left that is more highly rated gets 1 more candy than the right
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
