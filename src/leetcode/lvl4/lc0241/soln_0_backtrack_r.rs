/// @author: Leon
/// https://leetcode.com/problems/different-ways-to-add-parentheses/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/different-ways-to-add-parentheses/solutions/1294189/easy-solution-faster-than-100-explained-with-diagrams/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let chs: Vec<char> = expression.chars().collect();
        return Self::dfs(&chs[..]);
    }
    fn dfs(chs: &[char]) -> Vec<i32>{
        let mut res: Vec<i32> = Vec::new();
        let mut is_digit = true;
        for (idx, &ch) in chs.iter().enumerate(){
            if !chs[idx].is_digit(10){
                is_digit = false;
                let left: Vec<i32> = Self::dfs(&chs[0..idx]);
                let right: Vec<i32> = Self::dfs(&chs[idx + 1..]);
                for l in left{
                    for &r in &right{
                        let val = Self::perform(l, r, chs[idx]);
                        res.push(val);
                    }
                }
            }
        }
        if is_digit{
            res.push(Self::convert(chs));
            // res.push(chs.into_iter().collect::<String>().parse::<i32>().unwrap());
        }
        return res;
    }
    fn perform(x: i32, y: i32, op: char) -> i32{
        return match op{
            '+' => x + y,
            '-' => x - y,
            '*' => x * y,
            _ => 0,
        }
    }
    fn convert(chs: &[char]) -> i32{
        let mut res: i32 = 0;
        for &ch in chs{
            res *= 10;
            res += ch as i32 - '0' as i32;
        }
        return res;
    }
}