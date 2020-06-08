// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut l1m = &l1;
        let mut l2m = &l2;
        
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut dummy_head;
        let empty = Box::new(ListNode::new(0));
        
        while (l1m.is_some() || l2m.is_some()) {
            let l1v = l1m.as_ref().unwrap_or(&empty).val;
            let l2v = l2m.as_ref().unwrap_or(&empty).val;
            let res = l1v + l2v + carry;
            let val:i32 = res % 10;
            carry = res / 10;
            
            l1m = &l1m.as_ref().unwrap_or(&empty).next;
            l2m = &l2m.as_ref().unwrap_or(&empty).next;
            
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(val)));
            tail = &mut tail.as_mut().unwrap().next;
        }
        if carry != 0 {
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(carry)));
            tail = &mut tail.as_mut().unwrap().next;
        }
        
        dummy_head.unwrap().next
        
    }
    
}
