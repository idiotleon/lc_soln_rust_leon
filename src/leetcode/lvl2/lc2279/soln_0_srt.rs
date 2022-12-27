/// @author: Leon
/// https://leetcode.com/problems/maximum-bags-with-full-capacity-of-rocks/
/// Time Compleixty:    O(`len_bs` * lg(`len_bs`))
/// Space Complexity:   O(`len_bs`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
        let len_bs: usize = capacity.len();
        let gaps: Vec<i32> = {
            let mut gaps: Vec<i32> = Vec::with_capacity(len_bs);
            for (idx, cap) in capacity.into_iter().enumerate() {
                gaps.push(cap - rocks[idx]);
            }
            gaps.sort();
            gaps
        };
        let mut cnt: i32 = 0;
        let mut sum: i32 = 0;
        for gap in gaps {
            sum += gap;
            if sum > additional_rocks {
                break;
            }
            cnt += 1;
        }
        return cnt;
    }
}
