/// @author: Leon
/// https://leetcode.com/problems/maximum-points-you-can-obtain-from-cards/
/// Time Complexity:    O(`k`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let len_cps: usize = card_points.len();
        let ku: usize = k as usize;
        if ku >= len_cps{
            return card_points.into_iter().sum();
        }
        let mut sum: i32 = {
            let mut sum: i32 = 0;
            for idx in 0..ku{
                sum += card_points[idx];
            }
            sum
        };
        let mut most = sum;
        for idx in 0..ku{
            sum -= card_points[ku - idx - 1];
            sum += card_points[len_cps - 1 - idx];
            most = std::cmp::max(most, sum);
        }
        most
    }
}