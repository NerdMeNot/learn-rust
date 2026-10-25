List<int> nums = [1, 2, 3];
var t = Total(nums);        // pass the list in
Console.WriteLine($"{t} [{string.Join(", ", nums)}]");   // nums is still ours

// IReadOnlyList: a promise that Total won't change it
static int Total(IReadOnlyList<int> xs)
{
    var sum = 0;
    foreach (var x in xs)
    {
        sum += x;
    }
    return sum;
}
