

fn main() {
    let list = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: None,
            })),
        })),
    }));
    let list2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode {
                val: 6,
                next: None,
            })),
        })),
    }));
    let answer = merge_two_lists(list, list2);
    println!("{:#?}", answer);
}

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

fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut values = Vec::new();
    let mut current = list1;

    while let Some(node) = current {
        values.push(node.val);
        current = node.next;
    }
    current = list2;
    while let Some(node) = current {
        values.push(node.val);
        current = node.next;
    }
    values.sort();

    let mut head = None;
    for &val in values.iter().rev() {
        let new_node = Box::new(ListNode {
            val,
            next: head,
        });
        head = Some(new_node)
    }

    head
    
}
