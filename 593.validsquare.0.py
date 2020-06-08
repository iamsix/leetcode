class Solution(object):
    def validSquare(self, p1, p2, p3, p4):
        """
        :type p1: List[int]
        :type p2: List[int]
        :type p3: List[int]
        :type p4: List[int]
        :rtype: bool
        """
        if p1 == p2 == p3 == p4:
            return False
        sl = sorted([p1, p2, p3, p4])
        
        
        import math
        d1 = math.sqrt((sl[0][1] - sl[1][1])**2 + (sl[0][0] - sl[1][0])**2) #left
        d2 = math.sqrt((sl[1][1] - sl[3][1])**2 + (sl[1][0] - sl[3][0])**2) #bottom
        d3 = math.sqrt((sl[3][1] - sl[2][1])**2 + (sl[3][0] - sl[2][0])**2) #top
        d4 = math.sqrt((sl[2][1] - sl[0][1])**2 + (sl[2][0] - sl[0][0])**2) #right

        sq1 = math.sqrt((sl[0][1] - sl[3][1])**2 + (sl[0][0] - sl[3][0])**2) #daigonls
        sq2 = math.sqrt((sl[1][1] - sl[2][1])**2 + (sl[1][0] - sl[2][0])**2) #daigon


        return sq1 == sq2 and d1==d2==d3==d4

        
        
        
