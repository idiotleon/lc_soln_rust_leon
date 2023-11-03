use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/moving-average-from-data-stream/
/// Time Complexity:
/// `new()`:    O(1)
/// `next()`:   O(1)
/// Space Complexity:   O(`size`)
struct MovingAverage {
    queue: VecDeque<i32>,
    sum: i32,
    size: usize,
}

#[allow(dead_code)]
impl MovingAverage {
    fn new(size: i32) -> Self {
        let size: usize = size as usize;
        MovingAverage {
            queue: VecDeque::with_capacity(size),
            sum: 0,
            size,
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        self.sum += val;
        self.queue.push_back(val);
        if self.queue.len() > self.size {
            if let Some(cur) = self.queue.pop_front() {
                self.sum -= cur;
            }
        }
        return (self.sum as f64) / (self.queue.len() as f64);
    }
}
