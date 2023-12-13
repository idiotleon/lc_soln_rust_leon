/// @author: Leon
/// https://leetcode.com/problems/online-stock-span/
/// Time Complexity:    O()
/// Space Complexity:   O()
#[allow(dead_code)]
struct StockSpanner {
    prices: Vec<i32>,
}

#[allow(dead_code)]
impl StockSpanner {
    fn new() -> Self {
        Self { prices: Vec::new() }
    }
    fn next(&mut self, price: i32) -> i32 {
        let len_ps: usize = self.prices.len();
        let mut cnt: i32 = 1;
        for idx in (0..len_ps).rev() {
            if self.prices[idx] > price {
                break;
            }
            cnt += 1;
        }
        self.prices.push(price);
        return cnt;
    }
}
