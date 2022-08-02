use std::collections::VecDeque;

/// @author: Leon
/// https://leetcode.com/problems/maximum-number-of-books-you-can-take/
/// Time Complexity:    O()
/// Space Complexity:   O()
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_books(books: Vec<i32>) -> i64 {
        let len_bs: usize = books.len();
        let mut deque: VecDeque<i64> = VecDeque::with_capacity(len_bs);
        let mut most: i64 = 0;
        let mut sum: i64 = 0;
        for book in books {
            let book: i64 = book as i64;
            let mut len: i64 = 0;
            'inner: while let Some(&top) = deque.back() {
                if top < book {
                    break 'inner;
                }
                deque.pop_back();
                len += 1;
                sum -= top;
                let prev = book - len;
                if prev > 0 {
                    sum += prev;
                    deque.push_front(prev);
                }
            }
            deque.push_back(book);
            sum += book;
            most = std::cmp::max(most, sum);
        }
        most
    }
    fn dfs(stk: &mut VecDeque<i32>, value: i32) {
        if stk.is_empty() {
            stk.push_back(value);
            return;
        }
        while let Some(&_top) = stk.back() {
            todo!();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_with_sample_test_1_should_return_expected() {
        let books: Vec<i32> = vec![8, 5, 2, 7, 9];
        let expected: i64 = 19;
        let actual = Solution::maximum_books(books);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_sample_test_2_should_return_expected() {
        let books: Vec<i32> = vec![7, 0, 3, 4, 5];
        let expected: i64 = 12;
        let actual = Solution::maximum_books(books);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_with_test_case_56_should_return_expected() {
        let books: Vec<i32> = vec![5, 5, 5];
        let expected: i64 = 12;
        let actual = Solution::maximum_books(books);
        assert_eq!(expected, actual);
    }
}
