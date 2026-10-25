var a = new List<int> { 1, 2, 3 };
var b = a;
b.Add(4);
Console.WriteLine($"a has {a.Count} items");
Console.WriteLine($"same object: {ReferenceEquals(a, b)}");
