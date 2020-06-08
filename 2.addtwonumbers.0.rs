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
        let mut l1m = l1;
        let mut l2m = l2;
        let mut vec = Vec::new();
        
        let mut head = None;
        while (l1m.is_some() || l2m.is_some()) {
            let l1c = l1m.unwrap_or(Box::new(ListNode::new(0)));
            let l2c = l2m.unwrap_or(Box::new(ListNode::new(0)));
            let res = l1c.val + l2c.val + carry;
            let val:i32 = res % 10;
            carry = res / 10;
            
            l1m = l1c.next;
            l2m = l2c.next;
            vec.push(val)   
        }
        if carry != 0 {
            vec.push(carry)
        }
        
        while !vec.is_empty() {
            let val = vec.pop().unwrap();
            let mut new_node = ListNode::new(val);
            new_node.next = head;
            head = Some(Box::new(new_node));
        }
        head
        
    }
    
}
