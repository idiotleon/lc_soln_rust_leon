/// @author: Leon
/// https://leetcode.com/problems/meeting-rooms-ii/
/// Time Complexity:    O(`RANGE`)
/// Space Complexity:   O(`RANGE`)
/// Note:
/// overlapped edge points are NOT counted,
/// eg: [[1, 2], [2, 3]] is NOT considered as overlapped
/// Reference:
/// https://leetcode.com/problems/my-calendar-iii/discuss/109556/JavaC++-Clean-Code/111457
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        const RANGE: i32 = 1e6 as i32 + 7;
        let timeline: Vec<i32> = {
            let mut timeline: Vec<i32> = vec![0; RANGE as usize];
            for interval in intervals {
                let start = interval[0];
                let end = interval[1];
                timeline[start as usize] += 1;
                timeline[end as usize] -= 1;
            }
            timeline
        };
        let mut most: i32 = 0;
        let mut cur_needed: i32 = 0;
        for num in timeline {
            cur_needed += num;
            most = std::cmp::max(most, cur_needed);
        }
        most
    }
}
