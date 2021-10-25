/// https://leetcode.com/problems/min-stack/
///
/// Time Complexities:
///     new():
///     push():
///     pop():
///     top():
///     get_min():
/// Space Complexity:   O(N)
///
/// Reference:
///
use std::collections::VecDeque;

#[allow(dead_code)]
struct MinStack {
    stack: VecDeque<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl MinStack {
    fn new() -> Self {
        Self {
            stack: VecDeque::new(),
        }
    }
    fn push(&mut self, val: i32) {
        if self.stack.is_empty() {
            self.stack.push_back((val, val));
        } else {
            let min = self.stack.back().unwrap().1;
            self.stack.push_back((val, std::cmp::min(val, min)));
        }
    }
    fn pop(&mut self) {
        self.stack.pop_back();
    }
    fn top(&mut self) -> i32 {
        self.stack.back().unwrap().0
    }
    fn get_min(&mut self) -> i32 {
        self.stack.back().unwrap().1
    }
}
