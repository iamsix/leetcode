# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

class Solution:
    def addTwoNumbers(self, l1, l2):
        #iterative version...
        if not l1:
            return l2
        if not l2:
            return l1
        
        carry = 0
        _l1 = l1
        _l2 = l2
        while _l1.next or _l2.next:
            if not _l1.next:
                _l1.next = ListNode(0)
            if not _l2.next:
                _l2.next = ListNode(0)
            x = (_l1.val if _l1 else 0) + (_l2.val if _l2 else 0)
            
            if x >= 10:
                _l1.next.val += 1
                x %= 10

            _l1.val = x
            
            _l1 = _l1.next
            _l2 = _l2.next
        _l1.val += _l2.val
        if _l1.val >= 10:
            _l1.next = ListNode(1)
            _l1.val %= 10
        
        
        return l1
