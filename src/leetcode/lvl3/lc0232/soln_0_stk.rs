use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/implement-queue-using-stacks/
/// Time Complexities:
///         new():      O(1)
///         push():     O(1)
///         pop():      O(N)
///         peek():     O(N)
///         empty():    O(1)
/// Space Complexity:   O(N)
/// Reference:
/// https://leetcode.com/problems/implement-queue-using-stacks/discuss/64206/Short-O(1)-amortized-C%2B%2B-Java-Ruby
#[allow(dead_code)]
struct MyQueue {
    output: VecDeque<i32>,
    input: VecDeque<i32>,
}

#[allow(dead_code)]
impl MyQueue {
    fn new() -> Self {
        MyQueue {
            output: VecDeque::new(),
            input: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.input.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        Self::peek(self);
        self.output.pop_back().unwrap()
    }

    fn peek(&mut self) -> i32 {
        if self.output.is_empty() {
            while let Some(top) = self.input.pop_back() {
                self.output.push_back(top);
            }
        }
        *self.output.back().unwrap()
    }

    fn empty(&self) -> bool {
        self.output.is_empty() && self.input.is_empty()
    }
}
