/// @author: Leon
/// https://leetcode.com/problems/design-hit-counter/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/discuss/interview-question/178662/Design-a-Hit-Counter/
const FIVE_MIN: i32 = 5 * 60;

#[allow(dead_code)]
struct HitCounter {
    tses: Vec<i32>,
    hits: Vec<i32>,
}

#[allow(dead_code)]
impl HitCounter {
    /** Initialize your data structure here. */
    fn new() -> Self {
        HitCounter {
            tses: vec![0; FIVE_MIN as usize],
            hits: vec![0; FIVE_MIN as usize],
        }
    }
    /** Record a hit.
    @param timestamp - The current timestamp (in seconds granularity). */
    fn hit(&mut self, timestamp: i32) {
        let idx = (timestamp % FIVE_MIN) as usize;
        if self.tses[idx] != timestamp {
            self.tses[idx] = timestamp;
            self.hits[idx] = 1;
        } else {
            self.hits[idx] += 1;
        }
    }
    /** Return the number of hits in the past 5 minutes.
    @param timestamp - The current timestamp (in seconds granularity). */
    fn get_hits(&mut self, timestamp: i32) -> i32 {
        let mut count: i32 = 0;
        for idx in 0..FIVE_MIN as usize {
            if timestamp - self.tses[idx] < 300 {
                count += self.hits[idx];
            }
        }
        count
    }
}
