var nums = new List<int> { 1, 2, 3 };
foreach (var n in nums)
{
    n *= 10;
}
Console.WriteLine(string.Join(", ", nums));
