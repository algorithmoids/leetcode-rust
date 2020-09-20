struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false
        }

        let x = format!("{}", x);
        let reversed: String = format!("{}", x).chars().into_iter().rev().collect();
        x == reversed
    }
}


#[test]
fn test() {
    assert_eq!(Solution::is_palindrome(123), false);
    assert_eq!(Solution::is_palindrome(12321), true);
}