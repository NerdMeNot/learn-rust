var v = new List<int>();
v.Add(10);
v.Add(20);

List<int> w = [1, 2, 3];      // collection expression
w.Add(4);

var part = w.GetRange(1, 2);  // items 1 and 2 (a copy)
Console.WriteLine($"[{string.Join(", ", v)}] [{string.Join(", ", w)}] [{string.Join(", ", part)}] len={w.Count}");
