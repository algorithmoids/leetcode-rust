#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub struct Solution {}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut pointers = lists.into_iter()
            .filter(|x| x.is_some())
            .collect::<Vec<_>>();

        let mut ordered = vec![];

        while !pointers.is_empty() {
            let mut min_i = 0;

            for (i, x) in pointers.iter().enumerate() {
                if x.as_ref().unwrap().val < pointers[min_i].as_ref().unwrap().val {
                    min_i = i;
                }
            }

            let smallest = pointers.swap_remove(min_i).unwrap();

            ordered.push(smallest.val);
            if let Some(next) = smallest.next {
                pointers.push(Some(next));
            }
        }

        return make_linked_list(ordered);
    }
}


pub fn make_linked_list(input: Vec<i32>) -> Option<Box<ListNode>> {
    let mut node = None;

    for x in input.into_iter().rev() {
        node = Some(Box::new(ListNode {val: x, next: node}))
    }

    node
}


#[test]
fn test() {
    let input = vec![vec![1,10], vec![2,12]].into_iter().map(make_linked_list).collect();
    let expected = make_linked_list(vec![1,2,10,12]);

    assert_eq!(Solution::merge_k_lists(input), expected);

    let input = vec![vec![], vec![2,6]].into_iter().map(make_linked_list).collect();
    let expected = make_linked_list(vec![2,6]);

    assert_eq!(Solution::merge_k_lists(input), expected);

    let input = vec![vec![3], vec![2,6]].into_iter().map(make_linked_list).collect();
    let expected = Some(Box::new(ListNode { val: 2, next: Some(Box::new(ListNode { val: 3, next: Some(Box::new(ListNode::new(6)))}))}));

    assert_eq!(Solution::merge_k_lists(input), expected);


    let input = vec![vec![]].into_iter().map(make_linked_list).collect();
    let expected = make_linked_list(vec![]);

    assert_eq!(Solution::merge_k_lists(input), expected);


    let input = vec![].into_iter().map(make_linked_list).collect();
    let expected = make_linked_list(vec![]);

    assert_eq!(Solution::merge_k_lists(input), expected);
}