var fs = new List<Func<int>>();
for (int i = 0; i < 3; i++) fs.Add(() => i);
Console.WriteLine("for:     " + string.Join(" ", fs.Select(f => f())));

var gs = new List<Func<int>>();
foreach (var v in new[] { 0, 1, 2 }) gs.Add(() => v);
Console.WriteLine("foreach: " + string.Join(" ", gs.Select(g => g())));
