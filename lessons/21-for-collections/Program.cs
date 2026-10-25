List<string> names = ["ana", "bo"];

foreach (var n in names)
{
    Console.WriteLine($"hi {n}");
}

List<int> nums = [1, 2, 3];
for (var i = 0; i < nums.Count; i++)   // foreach can't change items, so use an index
{
    nums[i] *= 10;
}

foreach (var (n, i) in nums.Select((n, i) => (n, i)))   // with index
{
    Console.WriteLine($"{i}: {n}");
}
Console.WriteLine($"{names.Count} names, [{string.Join(", ", nums)}]");
