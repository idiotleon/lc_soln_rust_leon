/// https://leetcode.com/problems/super-palindromes/
/// 
/// Time Complexity:        O()
/// Space Complexity:       O()
///
/// Reference:
/// https://leetcode.com/problems/super-palindromes/discuss/1197431/Rust-backtracking-solution
use std::ops::RangeInclusive;

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn superpalindromes_in_range(left: String, right: String) -> i32 {
        let mut count = 0;
        if let (Ok(l), Ok(r)) = (left.parse::<u64>(), right.parse::<u64>()){
            {
                let mut v = Vec::with_capacity(6);
                Self::backtrack(&mut v, &(l..=r), false, &mut count);
            }
            {
                let mut v = Vec::with_capacity(6);
                Self::backtrack(&mut v, &(l..=r), true, &mut count);
            }
        }
        count
    }
    
    fn backtrack(v: &mut Vec<u64>, range: &RangeInclusive<u64>, is_odd: bool, count: &mut i32){
        if v.len() == 6{
            let digits = if let Some(pos) =  v.iter().position(|&d| d != 0){
                &v[pos..]
            }else{
                &v
            };
            let mut num = 0;
            for &d in digits.iter(){
                num = num * 10 + d;
            }
            for &d in digits.iter().rev().skip(if is_odd{ 1 } else { 0 }){
                num = num * 10 + d;
            }
            if range.contains(&(num.saturating_mul(num))){
                *count += 1;
            }
        }else{
            let sum = v.iter().map(|d| d * d).sum::<u64>();
            for u in 0..=9{
                if sum * 2 + u * u * if is_odd {1} else {2} < 10{
                    v.push(u);
                    Self::backtrack(v, range, is_odd, count);
                    v.pop();
                }
            }
        }
    }
}