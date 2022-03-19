use std::collections::{HashMap, VecDeque};
/// @author: Leon
/// https://leetcode.com/problems/maximum-frequency-stack/
/// Time Complexity:    O(1)
/// Space Complexity:   O(N)
/// Reference:
/// https://leetcode.com/problems/maximum-frequency-stack/discuss/163410/C%2B%2BJavaPython-O(1)
struct FreqStack {
    num_to_freq: HashMap<i32, i32>,
    num_to_stk: HashMap<i32, VecDeque<i32>>,
    max_freq: i32,
}

#[allow(dead_code)]
impl FreqStack {
    fn new() -> Self {
        FreqStack {
            num_to_freq: HashMap::new(),
            num_to_stk: HashMap::new(),
            max_freq: 0,
        }
    }

    fn push(&mut self, num: i32) {
        let freq = self.num_to_freq.entry(num).or_default();
        *freq += 1;
        self.max_freq = std::cmp::max(self.max_freq, *freq);
        self.num_to_stk
            .entry(self.max_freq)
            .or_default()
            .push_back(num);
    }

    fn pop(&mut self) -> i32 {
        let num = self
            .num_to_stk
            .entry(self.max_freq)
            .or_default()
            .pop_back()
            .unwrap();
        self.num_to_freq.insert(num, self.max_freq - 1);
        if self.num_to_stk.get(&self.max_freq).unwrap().is_empty() {
            self.max_freq -= 1;
        }
        num
    }
}
