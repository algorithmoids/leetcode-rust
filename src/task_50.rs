struct Solution {}

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut v = x;
        if n < 0 {
            v = 1_f64 / x;
        }

        let mut positive = (n as i64).abs();
        let mut result = 1_f64;
        let mut bin_power = v;

        while positive > 0 {
            if positive & 1 == 1 {
                result *= bin_power;
            }

            bin_power *= bin_power;

            positive >>= 1
        }

        result
    }
}


#[test]
fn test() {
    assert_eq!(Solution::my_pow(2_f64, 3), 8_f64);
}
