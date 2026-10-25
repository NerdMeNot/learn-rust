// A collection initializer is part of the language: no macro needed
var ages = new Dictionary<string, int> { ["Ada"] = 36, ["Linus"] = 28, ["Grace"] = 45 };

foreach (var name in ages.Keys.Order())
{
    Console.WriteLine($"{name}: {ages[name]}");
}
Console.WriteLine($"entries: {ages.Count}");
