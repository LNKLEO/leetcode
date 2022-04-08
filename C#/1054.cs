public class Solution
{
    public int[] RearrangeBarcodes(int[] barcodes)
    {
        List<int> answer = new(barcodes.Length);
        Dictionary<int, int> count = new();
        foreach (int code in barcodes)
        {
            if (!count.ContainsKey(code))
            {
                count.Add(code, 0);
            }
            --count[code];
        }
        SortedDictionary<int, List<int>> reverse = new();
        foreach (var record in count)
        {
            if (!reverse.ContainsKey(record.Value))
            {
                reverse.Add(record.Value, new());
            }
            reverse[record.Value].Add(record.Key);
        }
        while (reverse.Count() > 1)
        {
            if (reverse.ElementAt(0).Value.Count == 1)
            {
                int lastIDX = reverse.Count - 1;
                answer.Add(reverse.ElementAt(0).Value[0]);
                answer.Add(reverse.ElementAt(lastIDX).Value[0]);
                if (reverse.ElementAt(lastIDX).Key == -1)
                {
                    reverse.ElementAt(lastIDX).Value.RemoveAt(0);
                }
                else
                {
                    if (!reverse.ContainsKey(reverse.ElementAt(lastIDX).Key + 1))
                    {
                        reverse.Add(reverse.ElementAt(lastIDX).Key + 1, new());
                    }
                    reverse[reverse.ElementAt(lastIDX).Key + 1].Add(reverse.ElementAt(lastIDX).Value[0]);
                    reverse[reverse.ElementAt(lastIDX).Key].Remove(reverse.ElementAt(lastIDX).Value[0]);
                }
                if (reverse.ElementAt(lastIDX).Value.Count == 0)
                {
                    reverse.Remove(reverse.ElementAt(lastIDX).Key);
                }
            }
            else
            {
                answer.AddRange(reverse.ElementAt(0).Value);
            }
            if (reverse.ElementAt(0).Key < -1)
            {
                if (!reverse.ContainsKey(reverse.ElementAt(0).Key + 1))
                {
                    reverse.Add(reverse.ElementAt(0).Key + 1, new());
                }
                reverse[reverse.ElementAt(0).Key + 1].AddRange(reverse.ElementAt(0).Value);
            }
            reverse.Remove(reverse.ElementAt(0).Key);
        }
        if (reverse.Count() == 1)
        {
            for (int i = 0; i > reverse.ElementAt(0).Key; --i)
            {
                answer.AddRange(reverse.ElementAt(0).Value);
            }
        }
        return answer.ToArray();
    }
}
