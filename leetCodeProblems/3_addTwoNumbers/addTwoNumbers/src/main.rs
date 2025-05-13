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
    add_two_numbers(Some(boxed_list_node1),Some(boxed_list_node2));
    println!("Hello, world!");
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let boxed_list_node = Box::new(ListNode {
        val: 3,
        next: None,
    });
    return Some(boxed_list_node);
}
