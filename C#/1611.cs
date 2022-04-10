public class Solution {
    public int MinimumOneBitOperations(int n) {
        return n < 2 ? n : (int)Math.Pow(2, (int)Math.Log(n, 2)) + MinimumOneBitOperations((3 << ((int)Math.Log(n, 2) - 1)) ^ n);
    }
}
