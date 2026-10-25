List<int> nums = [1, 2, 3, 4, 5, 6];

var evensSquared = nums
    .Where(n => n % 2 == 0)
    .Select(n => n * n)
    .ToList();                  // build a List from the results

var total = nums.Sum();
Console.WriteLine($"[{string.Join(", ", evensSquared)}] {total}");
