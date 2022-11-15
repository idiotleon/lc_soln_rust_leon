/// @author: Leon
/// Time Complexity:    O(min(`n1`, `n2`))
/// Space Complexity:   O(1)
/// Reference:
/// https://www.baeldung.com/java-greatest-common-divisor
struct GCDBrutalForce;

#[allow(dead_code)]
impl GCDBrutalForce{
    fn get_gcd(n1: i32, n2: i32) -> i32{
        let mut gcd = 1;
        for num in 1..=std::cmp::min(n1, n2){
            if n1 % num == 0 && n2 % num == 0{
                gcd = num;
            }
        }
        return gcd;
    }
}