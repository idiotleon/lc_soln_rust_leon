/// https://leetcode.com/problems/design-hit-counter/
///
/// Time Complexity:    O()
/// Space Complexity:   O()
///
/// Reference:
/// https://leetcode.com/discuss/interview-question/178662/Design-a-Hit-Counter/
use std::collections::VecDeque;

const FIVE_MIN: i32 = 5 * 60;

#[allow(dead_code)]
struct HitCounter {
    queue: VecDeque<i32>,
}

#[allow(dead_code)]
impl HitCounter {
    /** Initialize your data structure here. */
    fn new() -> Self {
        HitCounter {
            queue: VecDeque::<i32>::new(),
        }
    }
    /** Record a hit.
    @param timestamp - The current timestamp (in seconds granularity). */
    fn hit(&mut self, timestamp: i32) {
        self.queue.push_back(timestamp)
    }
    /** Return the number of hits in the past 5 minutes.
    @param timestamp - The current timestamp (in seconds granularity). */
    fn get_hits(&mut self, timestamp: i32) -> i32 {
        while let Some(ts) = self.queue.front() {
            if timestamp - ts >= FIVE_MIN {
                self.queue.pop_front();
            } else {
                break;
            }
        }
        self.queue.len() as i32
    }
}
