/// @author: Leon
/// https://leetcode.com/problems/maximum-area-of-a-piece-of-cake-after-horizontal-and-vertical-cuts/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/maximum-area-of-a-piece-of-cake-after-horizontal-and-vertical-cuts/discuss/1248666/Rust-solution/960442
/// https://leetcode.com/problems/maximum-area-of-a-piece-of-cake-after-horizontal-and-vertical-cuts/discuss/1248666/Rust-solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_area(
        h: i32,
        w: i32,
        mut horizontal_cuts: Vec<i32>,
        mut vertical_cuts: Vec<i32>,
    ) -> i32 {
        const MOD: i64 = 1e9 as i64 + 7;
        let get_max_gap = |cuts: &mut Vec<i32>, dim| {
            cuts.extend(&[0, dim]);
            cuts.sort_unstable();
            cuts.windows(2).map(|w| w[1] - w[0]).max().unwrap() as i64
        };
        (get_max_gap(&mut horizontal_cuts, h) * get_max_gap(&mut vertical_cuts, w) % MOD) as _
    }
}
