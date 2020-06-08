# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def addTwoNumbers(self, l1, l2):
        #iterative version...
        res = _res = ListNode(0)
        carry = 0
        _l1 = l1
        _l2 = l2
        while _l1 or _l2 or carry:
            x = (_l1.val if _l1 else 0) + (_l2.val if _l2 else 0) + carry
            
            carry = x // 10
            x = x % 10
            _res.val = x
            
            _l1 = _l1.next if _l1 else None
            _l2 = _l2.next if _l2 else None
            
            if _l1 or _l2 or carry:
                _res.next = ListNode(0)
                _res = _res.next
        
        return res
