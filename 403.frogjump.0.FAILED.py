class Solution:
    def canCross(self, stones):
        """
        :type stones: List[int]
        :rtype: bool
        """
        lstone = 0
        ljump = 0
        possible = False
        for stone in stones:
            dist = stone - lstone
            print(dist)
            if dist == ljump or dist == ljump + 1 or dist == ljump - 1:
                possible = True
                lstone = stone
                ljump = dist
            else:
                return False
        return possible
