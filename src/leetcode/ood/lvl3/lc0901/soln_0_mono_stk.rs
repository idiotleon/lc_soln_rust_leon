use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/online-stock-span/
/// Time Complexity:    O()
/// Space Complexity:   O()
#[allow(dead_code)]
struct StockSpanner {
    stk: VecDeque<(i32, i32)>,
}

#[allow(dead_code)]
impl StockSpanner {
    fn new() -> Self {
        Self {
            stk: VecDeque::new(),
        }
    }
    fn next(&mut self, price: i32) -> i32 {
        let mut freq: i32 = 1;
        while let Some(&(price_top, freq_top)) = self.stk.back() {
            if price_top <= price {
                freq += freq_top;
                self.stk.pop_back();
            } else {
                break;
            }
        }
        self.stk.push_back((price, freq));
        return freq;
    }
}
