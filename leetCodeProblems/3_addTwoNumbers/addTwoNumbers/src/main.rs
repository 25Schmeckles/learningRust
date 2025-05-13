// Definition for singly-linked list.
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
fn main() {
    let boxed_list_node1 = Box::new(ListNode {
        val: 1,
        next: None,
    });
    let boxed_list_node2 = Box::new(ListNode {
        val: 2,
        next: None,
    });
    let return_added_numbers = add_two_numbers(Some(boxed_list_node1),Some(boxed_list_node2));
    if let Some(return_unwrap) = &return_added_numbers {
        println!("{}",&return_unwrap.val);
    }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
   let mut added_number = 0;
   let next_node = None;
    if let Some(l1_unwrap) = &l1 {
        added_number = added_number + &l1_unwrap.val;
    }
    if let Some(l2_unwrap) = &l2 {
        added_number = added_number + &l2_unwrap.val;
    }
    let boxed_list_node = Box::new(ListNode {
        val: added_number,
        next: next_node,
    });
    return Some(boxed_list_node);
}