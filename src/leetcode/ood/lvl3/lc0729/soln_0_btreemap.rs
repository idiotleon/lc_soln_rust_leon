use std::collections::BTreeMap;

/// @author: Leon
/// https://leetcode.com/problems/my-calendar-i/
/// Time Complexity:
///         `new()`:    O(1)
///         `book()`:   O(lg(N))
/// Space Complexity:   O(N)
#[allow(dead_code)]
struct MyCalendar {
    map: BTreeMap<i32, i32>,
}

#[allow(dead_code)]
impl MyCalendar {
    fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }
    fn book(&mut self, start: i32, end: i32) -> bool {
        if let Some((&_top_start, &top_end)) = self.map.range(..end).next_back() {
            if top_end > start {
                return false;
            }
        }
        self.map.insert(start, end);
        return true;
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
