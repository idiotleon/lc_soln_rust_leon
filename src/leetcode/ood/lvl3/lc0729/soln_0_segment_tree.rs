use std::cell::RefCell;
use std::rc::Rc;

/// @author: Leon
/// https://leetcode.com/problems/my-calendar-i/
/// Time Complexities:
///     `new()`:        O(lg(`RANGE`))
///     `book()`:       O()
/// Space Complexity:   O()
#[allow(dead_code)]
struct MyCalendar {
    root: Rc<RefCell<SegmentTree>>,
}

#[allow(dead_code)]
impl MyCalendar {
    fn new() -> Self {
        Self {
            root: Rc::new(RefCell::new(SegmentTree::new())),
        }
    }
    fn book(&mut self, start: i32, end: i32) -> bool {
        let k = self.root.clone().borrow_mut().query(start, end - 1);
        if k > 0 {
            return false;
        }
        self.root.clone().borrow_mut().update(start, end - 1, 1);
        return true;
    }
}

const RANGE: i32 = 1e9 as i32 + 7;

struct SegmentTree {
    root: Rc<RefCell<SegmentTreeNode>>,
}

impl SegmentTree {
    fn new() -> Self {
        Self {
            root: Rc::new(RefCell::new(SegmentTreeNode::new(0, RANGE))),
        }
    }
    fn query(&mut self, range_lo: i32, range_hi: i32) -> i32 {
        return Self::query_r(self, range_lo, range_hi, Some(self.root.clone()));
    }
    fn update(&mut self, range_lo: i32, range_hi: i32, freq: i32) {
        Self::update_r(self, range_lo, range_hi, freq, Some(self.root.clone()));
    }
    fn query_r(
        &mut self,
        range_lo: i32,
        range_hi: i32,
        node: Option<Rc<RefCell<SegmentTreeNode>>>,
    ) -> i32 {
        if let Some(n) = node {
            Self::push_down(self, n.clone());
            if range_lo > n.borrow().hi || range_hi < n.borrow().lo {
                return 0;
            }
            if range_lo <= n.borrow().lo && n.borrow().hi <= range_hi {
                return n.borrow().max_freq;
            }
            let left_max_freq = Self::query_r(self, range_lo, range_hi, n.borrow().left.clone());
            let right_max_freq = Self::query_r(self, range_lo, range_hi, n.borrow().right.clone());
            return std::cmp::max(left_max_freq, right_max_freq);
        } else {
            return 0;
        }
    }
    fn update_r(
        &mut self,
        range_lo: i32,
        range_hi: i32,
        value: i32,
        node: Option<Rc<RefCell<SegmentTreeNode>>>,
    ) {
        if let Some(n) = node {
            let lo: i32 = n.borrow().lo;
            let hi: i32 = n.borrow().hi;
            let left = n.borrow().left.clone();
            let right = n.borrow().right.clone();
            if range_lo <= lo && hi <= range_hi {
                n.borrow_mut().lazy += value;
            }
            Self::push_down(self, n.clone());
            if range_lo <= lo && hi <= range_hi || range_lo > hi || range_hi < lo {
                return;
            }
            Self::update_r(self, range_lo, range_hi, value, left.clone());
            Self::update_r(self, range_lo, range_hi, value, right.clone());
            let max_freq = std::cmp::max(
                left.clone().unwrap().borrow().max_freq,
                right.clone().unwrap().borrow().max_freq,
            );
            n.borrow_mut().max_freq = max_freq;
        }
    }
    fn push_down(&self, node: Rc<RefCell<SegmentTreeNode>>) {
        let mut n = node.borrow_mut();
        let lo: i32 = n.lo;
        let hi: i32 = n.hi;
        let lazy: i32 = n.lazy;
        n.max_freq += lazy;
        if lo != hi {
            let mid = lo + (hi - lo) / 2;
            if n.left.is_none() {
                n.left = Some(Rc::new(RefCell::new(SegmentTreeNode::new(lo, mid))));
            }
            if n.right.is_none() {
                n.right = Some(Rc::new(RefCell::new(SegmentTreeNode::new(1 + mid, hi))));
            }
            n.left.clone().unwrap().borrow_mut().lazy += lazy;
            n.right.clone().unwrap().borrow_mut().lazy += lazy;
        }
        n.lazy = 0;
    }
}

struct SegmentTreeNode {
    lo: i32,
    hi: i32,
    max_freq: i32,
    lazy: i32,
    left: Option<Rc<RefCell<SegmentTreeNode>>>,
    right: Option<Rc<RefCell<SegmentTreeNode>>>,
}

impl SegmentTreeNode {
    pub fn new(lo: i32, hi: i32) -> Self {
        Self {
            lo,
            hi,
            max_freq: 0,
            lazy: 0,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let mut calendar = MyCalendar::new();
        let expected0: bool = true;
        let actual0: bool = calendar.book(10, 20);
        assert_eq!(expected0, actual0);
        let expected1: bool = false;
        let actual1: bool = calendar.book(15, 20);
        assert_eq!(expected1, actual1);
        let expected2: bool = true;
        let actual2: bool = calendar.book(20, 30);
        assert_eq!(expected2, actual2);
    }
}
