use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/max-stack/description/
/// Time Complexities:
///
/// Space Complexity:   O(N)
/// Reference:
/// https://leetcode.com/problems/max-stack/solutions/108938/java-ac-solution/
#[allow(dead_code)]
struct MaxStack {
    stk: VecDeque<i32>,
    stk_max: VecDeque<i32>,
}

#[allow(dead_code)]
impl MaxStack {
    fn new() -> Self {
        Self {
            stk: VecDeque::new(),
            stk_max: VecDeque::new(),
        }
    }
    fn push(&mut self, x: i32) {
        Self::push_helper(self, x);
    }
    fn pop(&mut self) -> i32 {
        self.stk_max.pop_back();
        return self.stk.pop_back().unwrap();
    }
    fn top(&self) -> i32 {
        return *self.stk.back().unwrap();
    }
    fn peek_max(&self) -> i32 {
        return *self.stk_max.back().unwrap();
    }
    fn pop_max(&mut self) -> i32 {
        let max: i32 = *self.stk_max.back().unwrap();
        let mut tmp: VecDeque<i32> = VecDeque::new();
        while let Some(&top) = self.stk.back() {
            if top != max {
                tmp.push_back(top);
                self.stk.pop_back();
                self.stk_max.pop_back();
            } else {
                break;
            }
        }
        self.stk.pop_back();
        self.stk_max.pop_back();
        while let Some(t) = tmp.pop_back() {
            Self::push_helper(self, t);
        }
        return max;
    }

    fn push_helper(&mut self, x: i32) {
        let mut tmp: i32 = if let Some(&max) = self.stk_max.back() {
            max
        } else {
            i32::MIN
        };
        if x > tmp {
            tmp = x;
        }
        self.stk.push_back(x);
        self.stk_max.push_back(tmp);
    }
}
