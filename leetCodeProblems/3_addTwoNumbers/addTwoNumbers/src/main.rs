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
    
    //loops through both lists simultenously and creates a new third list
    while let (Some(l1_traverse), Some(l2_traverse)) = (currentl1, currentl2) {
        let added_numbers = Box::new(ListNode {
            val: l1_traverse.val + l2_traverse.val,
            next: None,
        });
        *tail_ref = Some(added_numbers);
        tail_ref = &mut tail_ref.as_mut().unwrap().next;
        //traverse next on the supplied lists
        currentl1 = l1_traverse.next;
        currentl2 = l2_traverse.next;
    }
    new_head
}