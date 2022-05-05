use std::collections::VecDeque;
/// @author: Leon
/// https://leetcode.com/problems/implement-stack-using-queues/
/// Time Complexities:
///     `push()`:   O(N)
///     `pop()`:    O(1)
///     `top()`:    O(1)
///     `empty`:    O(1)
/// Space Complexity: O(N)
/// Reference:
/// https://leetcode.com/problems/implement-stack-using-queues/discuss/62516/Concise-1-Queue-Java-C%2B%2B-Python
#[allow(dead_code)]
struct MyStack {
    queue: VecDeque<i32>,
}

#[allow(dead_code)]
impl MyStack {
    fn new() -> Self {
        MyStack {
            queue: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.queue.push_back(x);
        for _ in 1..self.queue.len() {
            if let Some(first) = self.queue.pop_front() {
                self.queue.push_back(first);
            }
        }
    }

    fn pop(&mut self) -> i32 {
        self.queue.pop_front().unwrap()
    }

    fn top(&self) -> i32 {
        *self.queue.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}
