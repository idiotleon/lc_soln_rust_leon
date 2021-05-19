/// https://leetcode.com/problems/logger-rate-limiter/
///
/// Time Complexity:    O(1)
/// Space Complexity:   O(N)
///
/// Reference:
/// https://leetcode.com/problems/logger-rate-limiter/discuss/944950/Rust-20ms-not-terrible-not-great-either
/// https://leetcode.com/problems/logger-rate-limiter/discuss/391558/Review-of-four-different-solutions%3A-HashMap-Two-Sets-Queue-with-Set-Radix-buckets-(Java-centric)
use std::cell::RefCell;
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Default)]
struct Logger {
    mes_to_ts: RefCell<HashMap<String, i32>>,
}

#[allow(dead_code)]
impl Logger {
    fn new() -> Self {
        Default::default()
    }
    fn should_print_message(&self, timestamp: i32, message: String) -> bool {
        if timestamp - self.mes_to_ts.borrow().get(&message).unwrap_or(&-10) < 10 {
            false
        } else {
            self.mes_to_ts.borrow_mut().insert(message, timestamp);
            true
        }
    }
}
