use std::iter::{Flatten, Peekable};
use std::vec::IntoIter;

/// @author: Leon
/// https://leetcode.com/problems/flatten-2d-vector/
/// Time Complexity:
///         `new()`:        O(N)
///         `next()`:       O(1)
///         `has_next()`:   O(1)
/// Space Complexity:       O(N)
#[allow(dead_code)]
struct Vector2D {
    it: Peekable<Flatten<IntoIter<Vec<i32>>>>,
}

#[allow(dead_code)]
impl Vector2D {
    fn new(vec: Vec<Vec<i32>>) -> Self {
        Self {
            it: vec.into_iter().flatten().peekable(),
        }
    }

    fn next(&mut self) -> i32 {
        self.it.next().unwrap()
    }

    fn has_next(&mut self) -> bool {
        self.it.peek().is_some()
    }
}
