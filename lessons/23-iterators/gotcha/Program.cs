List<int> nums = [1, 2, 3, 4, 5, 6];
var evens = nums.Where(n => n % 2 == 0).ToList();
Console.WriteLine($"[{string.Join(", ", evens)}]");
