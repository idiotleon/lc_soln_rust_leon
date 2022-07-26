use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/min-stack/
/// Time Complexities:
///     new():
///     push():
///     pop():
///     top():
///     get_min():
/// Space Complexity:   O(N)
struct MinStack {
    stk: VecDeque<(i32, i32)>,
}

#[allow(dead_code)]
impl MinStack {
    fn new() -> Self {
        Self {
            stk: VecDeque::new(),
        }
    }

    fn push(&mut self, val: i32) {
        if let Some(&(_num, min)) = self.stk.back() {
            self.stk.push_back((val, std::cmp::min(val, min)));
        } else {
            self.stk.push_back((val, val));
        }
    }

    fn pop(&mut self) {
        if let Some((_val, _min)) = self.stk.pop_back() {
            // to purposefully do nothing
            return;
        } else {
            unreachable!();
        }
    }

    fn top(&mut self) -> i32 {
        if let Some(&(val, _min)) = self.stk.back() {
            return val;
        } else {
            unreachable!();
        }
    }

    fn get_min(&mut self) -> i32 {
        if let Some(&(_val, min)) = self.stk.back() {
            return min;
        } else {
            unreachable!();
        }
    }
}
