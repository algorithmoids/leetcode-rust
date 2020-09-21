use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut subs = HashMap::new();

        for i in 0 .. nums.len() {
            let num = nums[i];
            let sub = target - nums[i];

            if subs.contains_key(&num) {
                return vec![i as i32, subs[&num]]
            }
            subs.insert(sub, i as i32);
        }

        panic!("no sum")
    }
}

#[test]
fn test() {
    assert_eq!(Solution::two_sum(vec![2,7,11,15], 9), vec![1, 0]);
}
