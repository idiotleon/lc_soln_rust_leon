/// @author: Leon
/// https://leetcode.com/problems/minimum-cost-of-buying-candies-with-discount/
/// Time Complexity:    O(`len_n` * lg(`len_n`))
/// Space Complexity:   O(1) / O(`len_n`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_cost(costs: Vec<i32>) -> i32 {
        let len_c: usize = costs.len();
        let sorted: Vec<i32> = {
            let mut sorted = costs;
            sorted.sort();
            sorted
        };
        let mut sum: i32 = 0;
        let mut steps = 0;
        for idx in (0..len_c).rev() {
            if steps == 2 {
                steps = 0;
                continue;
            }
            sum += sorted[idx];
            steps += 1;
        }
        sum
    }
}
