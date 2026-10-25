List<int> a = [1, 2, 3];
var b = a;              // b points at the SAME list. a still works too.
Console.WriteLine($"[{string.Join(", ", b)}]");

var x = 5;
var y = x;              // numbers are copied
Console.WriteLine($"{x} {y}");
