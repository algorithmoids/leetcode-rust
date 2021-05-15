#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
  Int(i32),
  List(Vec<NestedInteger>)
}

struct Solution;

impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        if !s.starts_with("[") {
            return NestedInteger::Int(s.parse().unwrap())
        }

        let mut arrays = vec![vec![]];
        let mut value: Option<String> = None;

        for char in s.chars() {
            match char {
                '[' => {
                    arrays.push(vec![]);
                },
                ']' => {
                    let mut array = arrays.pop().unwrap();

                    if let Some(number) = value {
                        array.push(NestedInteger::Int(number.parse().unwrap()))
                    }
                    value = None;

                    arrays.last_mut().unwrap().push(NestedInteger::List(array));
                },
                ',' => {
                    if let Some(number) = value {
                        arrays.last_mut().unwrap().push(NestedInteger::Int(number.parse().unwrap()))
                    }
                    value = None;
                },
                digit => {
                    value = value.map(|x| format!("{}{}", x, digit)).or(Some(digit.to_string()));
                }
            }
        }

        return arrays.pop().unwrap().pop().unwrap();
    }
}


#[test]
fn test() {
    assert_eq!(NestedInteger::Int(123), Solution::deserialize(String::from("123")));
    assert_eq!(NestedInteger::List(vec![NestedInteger::Int(123)]), Solution::deserialize(String::from("[123]")));

    assert_eq!(
        NestedInteger::List(vec![
            NestedInteger::Int(123),
            NestedInteger::List(vec![
                NestedInteger::Int(456),
                NestedInteger::List(vec![
                    NestedInteger::Int(789),
                    NestedInteger::Int(-11)
                ]),
                NestedInteger::Int(12),
            ]),
        ]),
        Solution::deserialize(String::from("[123,[456,[789,-11],12]]")));
}
