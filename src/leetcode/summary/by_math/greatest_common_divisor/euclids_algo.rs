/// @author: Leon
/// Time Complexity:    O(lg(min(`n1`, `n2`)))
/// Space Complexity:   O(lg(min(`n1`, `n2`)))
/// Rerference:
/// https://www.baeldung.com/java-greatest-common-divisor
struct GCDEuclidsAlgorithm;

#[allow(dead_code)]
impl GCDEuclidsAlgorithm {
    fn get_gcd(n1: i32, n2: i32) -> i32 {
        if n2 == 0 {
            return n1;
        }
        return Self::get_gcd(n2, n1 % n2);
    }
}
