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
    let mut l1nums = vec![];
    let mut l2nums = vec![];
    //loops through both lists simultenously collecting the values and padding with 0s in case of unever list sizes
    while currentl1.is_some() || currentl2.is_some() {
        if let Some(node) = currentl1 {
            l1nums.push(node.val);
            currentl1 = node.next;
        } else {
            l1nums.push(0);
        }
        if let Some(node) = currentl2 {
            l2nums.push(node.val);
            currentl2 = node.next;
        } else {
            l2nums.push(0);
        }
    }
    //reverse each vector, turn it into an integer, add, then convert back to vec and reverse
    l1nums.reverse();
    let l1num: u128 = l1nums.iter().fold(0, |acc, &d| acc * 10 + d as u128);
    l2nums.reverse();
    let l2num: u128 = l2nums.iter().fold(0, |acc, &d| acc * 10 + d as u128);
    let added_nums: u128 = l1num as u128 + l2num as u128;
    let mut added_nums: Vec<i32> = added_nums
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();
    added_nums.reverse();
    //for creating/traversing our new output list
    let mut new_head: Option<Box<ListNode>> = None;
    let mut tail_ref = &mut new_head;
    //create new list from our vector and return its head
    for number in added_nums.iter() {
        let added_numbers = Box::new(ListNode {
        val: *number,
        next: None,
        });
        *tail_ref = Some(added_numbers);
        tail_ref = &mut tail_ref.as_mut().unwrap().next;
    }
    new_head
}