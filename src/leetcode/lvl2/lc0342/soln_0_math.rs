struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        let mut n = n;
        if n > 1{
            while n % 4 == 0{
                n /= 4;
            }
        }
        return n == 1;
    }
}