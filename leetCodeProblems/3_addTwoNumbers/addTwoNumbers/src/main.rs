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
    let mut currentl1 = l1;
    let mut currentl2 = l2;
    let mut new_head: Option<Box<ListNode>> = None;
    let mut tail_ref = &mut new_head;
    let mut carry = 0;
    //loops through until you reach the end of both lists and carry is 0
    //at each step it conducts least significant digit (LSD) addition
    //then it links the list and returns the head
    while currentl1.is_some() || currentl2.is_some() || carry > 0 {
        let mut x = 0;
        let mut y = 0;
        //gets value from nodes
        if let Some(node) = currentl1 {
            x = node.val;
            currentl1 = node.next;
        }
        if let Some(node) = currentl2 {
            y = node.val;
            currentl2 = node.next;
        }
        let sum = x + y + carry;
        let added_numbers = Box::new(ListNode::new(sum % 10));
        carry = sum / 10;
        *tail_ref = Some(added_numbers);
        tail_ref = &mut tail_ref.as_mut().unwrap().next;
    }
    new_head
}