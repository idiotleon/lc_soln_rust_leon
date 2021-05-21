/// https://leetcode.com/problems/design-hit-counter/
///
/// Time Complexity:    O()
/// Space Complexity:   O()
///
/// Reference:
/// https://leetcode.com/discuss/interview-question/178662/Design-a-Hit-Counter/
const FIVE_MIN: i32 = 5 * 60;

#[allow(dead_code)]
struct HitCounter {
    tses: Vec<i32>,
}

#[allow(dead_code)]
impl HitCounter {
    /** Initialize your data structure here. */
    fn new() -> Self {
        HitCounter { tses: Vec::new() }
    }
    /** Record a hit.
    @param timestamp - The current timestamp (in seconds granularity). */
    fn hit(&mut self, ts: i32) {
        self.tses.push(ts);
    }
    /** Return the number of hits in the past 5 minutes.
    @param timestamp - The current timestamp (in seconds granularity). */
    fn get_hits(&self, timestamp: i32) -> i32 {
        for (idx, &ts) in self.tses.iter().enumerate() {
            if ts > timestamp - FIVE_MIN {
                return (self.tses.len() - idx) as i32;
            }
        }
        0
    }
}
