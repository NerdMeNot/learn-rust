const string name = "gopher";   // cannot change
var count = 1;                  // var: can change
count += 1;

// C# does not let a local reuse a local's name, so the new value gets a new name
var total = count * 10;
Console.WriteLine($"{name} {total}");
