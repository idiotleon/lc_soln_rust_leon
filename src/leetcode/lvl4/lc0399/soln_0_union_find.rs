use std::collections::HashMap;

/// @author: Leon
/// https://leetcode.com/problems/evaluate-division/
/// Time Complexity:    O()
/// Space Complexity:   O()
/// Reference:
/// https://leetcode.com/problems/evaluate-division/discuss/147281/Java-Union-Find-solution-faster-than-99
/// Note:
/// this is not yet a correct solution
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let _len_eqs: usize = equations.len();
        let len_qs: usize = queries.len();
        let mut roots: HashMap<String, String> = HashMap::new();
        let mut dist: HashMap<String, f64> = HashMap::new();
        for (idx, equation) in equations.iter().enumerate() {
            let numerator = &equation[0].as_str();
            let divisor = &equation[1].as_str();
            Self::union(numerator, divisor, &mut roots, &mut dist, values[idx]);
            println!("roots: {:?}", roots);
            println!("dist: {:?}", dist);
        }
        let mut ans: Vec<f64> = vec![0.0; len_qs];
        for (idx, query) in queries.iter().enumerate() {
            let numerator = &query[0];
            let divisor = &query[1];
            if !roots.contains_key(numerator)
                || !roots.contains_key(divisor)
                || !Self::find(numerator, &mut roots, &mut dist)
                    .eq(&Self::find(divisor, &mut roots, &mut dist))
            {
                ans[idx] = -1.0;
            } else {
                ans[idx] = dist.get(numerator).unwrap() / dist.get(divisor).unwrap();
            }
        }
        ans
    }
    fn union(
        numerator: &str,
        divisor: &str,
        roots: &mut HashMap<String, String>,
        dist: &mut HashMap<String, f64>,
        value: f64,
    ) {
        roots
            .entry(numerator.to_owned())
            .or_insert(numerator.to_owned());
        dist.entry(numerator.to_owned()).or_insert(1.0);
        roots
            .entry(divisor.to_owned())
            .or_insert(divisor.to_owned());
        dist.entry(divisor.to_owned()).or_insert(1.0);
        let root_n: String = Self::find(numerator, roots, dist);
        let root_d: String = Self::find(divisor, roots, dist);
        println!("{:?}", roots);
        println!("{:?}", dist);
        println!("root_n: {:?}", root_n);
        println!("root_d: {:?}", root_d);
        roots.insert(root_n.to_owned(), root_d.to_owned());
        dist.insert(
            root_n.to_owned(),
            value * dist.get(&root_d.to_owned()).unwrap() / dist.get(&root_n).unwrap(),
        );
        println!("{:?}", roots);
        println!("{:?}", dist);
    }
    fn find(
        x: &str,
        roots: &mut HashMap<String, String>,
        dist: &mut HashMap<String, f64>,
    ) -> String {
        if !roots.contains_key(x) {
            roots.insert(x.to_owned(), x.to_owned());
            dist.insert(x.to_owned(), 1.0);
            return x.to_owned();
        }
        let rt = roots.get(x).unwrap().to_owned();
        if rt.eq(&x.to_owned()) {
            return x.to_owned();
        }
        let rt_grand = Self::find(&rt, roots, dist);
        roots.insert(rt.to_owned(), rt_grand.to_owned());
        dist.insert(
            rt.to_owned(),
            dist.get(x).unwrap() * dist.get(&rt.to_owned()).unwrap(),
        );
        return rt_grand;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_input_1_should_return_expected() {
        let equations: Vec<Vec<String>> = vec![
            vec!["a".to_owned(), "b".to_owned()],
            vec!["b".to_owned(), "c".to_owned()],
        ];
        let values: Vec<f64> = vec![2.0, 3.0];
        let queries: Vec<Vec<String>> = vec![
            vec!["a".to_owned(), "c".to_owned()],
            vec!["b".to_owned(), "a".to_owned()],
            vec!["a".to_owned(), "e".to_owned()],
            vec!["a".to_owned(), "a".to_owned()],
            vec!["x".to_owned(), "x".to_owned()],
        ];
        let expected: Vec<f64> = vec![6.00000, 0.50000, -1.00000, 1.00000, -1.00000];
        let actual: Vec<f64> = Solution::calc_equation(equations, values, queries);
        assert_eq!(expected, actual);
    }
}
