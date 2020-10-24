/*
2. Add Two Numbers

You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order and each of their nodes contain a single digit. Add the two numbers and return it as a linked list.

You may assume the two numbers do not contain any leading zero, except the number 0 itself.

Example:

Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
Output: 7 -> 0 -> 8
Explanation: 342 + 465 = 807.
*/


//Definition for singly-linked list.
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
  
fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

    fn add_two_numbers_rec(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {

        fn add (v1: i32, v2: i32, carry: i32) -> (i32, i32) {
            let sum = v1 + v2 + carry;
            (sum % 10, sum / 10)
        }

        fn next (val: i32, carry: i32, n1: Option<Box<ListNode>>, n2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut node = ListNode::new(val);
            node.next = add_two_numbers_rec(n1, n2, carry);
            return Some(Box::new(node))
        }

        match (list1, list2) {
            (Some(list1), Some(list2)) => {
                let (val, carry) = add(list1.val, list2.val, carry);
                return next(val, carry, list1.next, list2.next);
            },
            (Some(list1), None) => {
                let (val, carry) = add(list1.val, 0, carry);
                return next (val, carry, list1.next, None);
            },
            (None, Some(list2)) => {
                let (val, carry) = add(0, list2.val, carry);
                return next (val, carry, None, list2.next);
            }
            (None, None) => {
                if carry > 0 {
                    return Some(Box::new(ListNode::new(carry)));
                }
                else { return None; }
            }
        }
    }

    add_two_numbers_rec(l1, l2, 0)
}

#[test]
fn add_two_numbers_test() {
    let a = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode {
            val: 4,
            next: Some(Box::new(ListNode {
                val: 3,
                next: None
            }))
        }))
    }));

    let b = Some(Box::new(ListNode {
        val: 5,
        next: Some(Box::new(ListNode {
            val: 6,
            next: Some(Box::new(ListNode {
                val: 4,
                next: None
            }))
        }))
    }));

    let expected = Some(Box::new(ListNode {
        val: 7,
        next: Some(Box::new(ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 8,
                next: None
            }))
        }))
    }));

    assert_eq!(add_two_numbers(a, b), expected);
}
