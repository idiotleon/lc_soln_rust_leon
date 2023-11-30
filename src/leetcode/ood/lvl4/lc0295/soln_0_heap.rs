use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[allow(dead_code)]
struct MedianFinder {
    is_even: bool,
    large: BinaryHeap<i32>,
    small: BinaryHeap<Reverse<i32>>,
}

#[allow(dead_code)]
impl MedianFinder {
    fn new() -> Self {
        Self {
            is_even: true,
            large: BinaryHeap::new(),
            small: BinaryHeap::new(),
        }
    }
    fn add_num(&mut self, num: i32) {
        if self.is_even {
            self.large.push(num);
            if let Some(l) = self.large.pop() {
                self.small.push(Reverse(l));
            }
        } else {
            self.small.push(Reverse(num));
            if let Some(Reverse(s)) = self.small.pop() {
                self.large.push(s);
            }
        }
        self.is_even = !self.is_even;
    }
    fn find_median(&self) -> f64 {
        return if self.is_even {
            (self.small.peek().unwrap().0 + *self.large.peek().unwrap()) as f64 / 2_f64
        } else {
            self.small.peek().unwrap().0 as f64
        };
    }
}
