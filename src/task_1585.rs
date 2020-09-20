use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn is_transformable(s: String, t: String) -> bool {
        let sorted: HashMap<_, _> = t.chars()
            .into_iter()
            .enumerate()
            .map(|(a, b)| (b, a))
            .collect();

        let mut positions: Vec<_> = s.chars()
            .into_iter()
            .map(|x| (sorted.get(&x).unwrap(), x))
            .collect();


        for number in 0 .. positions.len() {
            for i in (0 .. positions.len()).rev() {
                if positions[i].0 == &number {
                    if i == number {
                        continue
                    }

                    if positions[i].1 > positions[i-1].1 {
                        return false
                    }

                    if positions[i].1 < positions[i-1].1 {
                        let t = positions[i];
                        positions[i] = positions[i - 1];
                        positions[i - 1] = t;
                    }

                }
            }
        }

        true
    }
}


#[test]
fn test_case() {
    assert!(Solution::is_transformable(String::from("1"), String::from("2")))
}
