struct Solution;

#[allow(dead_code)]
impl Solution {
    const BY_DAY: i32 = 1;
    const BY_WEEK: i32 = 7;
    const BY_MONTH: i32 = 30;
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let len_ds = days.len();
        let mut memo: Vec<Option<i32>> = vec![None; len_ds];
        return Self::dfs(0, &days, &costs, &mut memo);
    }
    fn dfs(idx: usize, days: &Vec<i32>, costs: &Vec<i32>, memo: &mut Vec<Option<i32>>) -> i32 {
        let len_ds = days.len();
        if idx == len_ds {
            return 0;
        }
        if let Some(m) = memo[idx] {
            return m;
        }
        let by_day = costs[0]
            + Self::dfs(
                Self::get_next_day(idx, Self::BY_DAY, days),
                days,
                costs,
                memo,
            );
        let by_week = costs[1]
            + Self::dfs(
                Self::get_next_day(idx, Self::BY_WEEK, days),
                days,
                costs,
                memo,
            );
        let by_month = costs[2]
            + Self::dfs(
                Self::get_next_day(idx, Self::BY_MONTH, days),
                days,
                costs,
                memo,
            );
        let min_cost = std::cmp::min(by_day, std::cmp::min(by_week, by_month));
        memo[idx] = Some(min_cost);
        return min_cost;
    }
    fn get_next_day(idx_cur: usize, duration: i32, days: &Vec<i32>) -> usize {
        let len_ds: usize = days.len();
        let nxt_end = days[idx_cur] + duration - 1;
        let mut idx = idx_cur;
        while idx < len_ds && days[idx] <= nxt_end {
            idx += 1;
        }
        return idx;
    }
}
