use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/flatten-2d-vector/
/// Time Complexity:
///         `new()`:        O(`len`)
///         `next()`:       O(1)
///         `has_next()`:   O(1)
/// Space Complexity:       O(`len`)
#[allow(dead_code)]
struct Vector2D {
    queue: VecDeque<i32>,
    idx: usize,
}

#[allow(dead_code)]
impl Vector2D {
    fn new(vec: Vec<Vec<i32>>) -> Self {
        let len_rs: usize = vec.len();
        let len: usize = vec.iter().map(|r| r.len()).sum();
        let mut queue: VecDeque<i32> = VecDeque::with_capacity(len);
        for r in 0..len_rs {
            for c in 0..vec[r].len() {
                queue.push_back(vec[r][c]);
            }
        }
        Self { queue, idx: 0 }
    }

    fn next(&mut self) -> i32 {
        if let Some(first) = self.queue.pop_front() {
            return first;
        }
        unreachable!();
    }

    fn has_next(&self) -> bool {
        !self.queue.is_empty()
    }
}
