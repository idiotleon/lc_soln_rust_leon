/// @author: Leon
/// https://leetcode.com/problems/meeting-rooms/
/// Time Complexity:    O(`RANGE`)
/// Space Complexity:   O(`RANGE`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
        const RANGE: i32 = 1e6 as i32 + 7;
        let mut timeline: Vec<i32> = vec![0; RANGE as usize];
        for interval in intervals {
            let start: usize = interval[0] as usize;
            let end: usize = interval[1] as usize;
            timeline[start] += 1;
            timeline[end] -= 1;
        }
        let mut sum: i32 = 0;
        for time in timeline {
            sum += time;
            if sum > 1 {
                return false;
            }
        }
        true
    }
}
