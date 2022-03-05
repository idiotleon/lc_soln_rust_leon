/// @author: Leon
/// https://leetcode.com/problems/delete-and-earn/
/// Time Complexity:    O(`RANGE`)
/// Space Complexity:   O(`RANGE`)
/// Reference:
/// https://leetcode.com/problems/delete-and-earn/discuss/109895/JavaC%2B%2B-Clean-Code-with-Explanation
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        const RANGE: usize = 1e5 as usize + 7;
        let freqs: Vec<i32> = {
            let mut freqs: Vec<i32> = vec![0; RANGE];
            for num in nums {
                freqs[num as usize] += num;
            }
            freqs
        };
        let mut take_prev: i32 = 0;
        let mut skip_prev: i32 = 0;
        for idx in 0..RANGE {
            let take_cur = skip_prev + freqs[idx];
            let skip_cur = std::cmp::max(skip_prev, take_prev);
            take_prev = take_cur;
            skip_prev = skip_cur;
        }
        std::cmp::max(take_prev, skip_prev)
    }
}
