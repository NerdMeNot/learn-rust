List<int> nums = [1, 2, 3];
var t = Total(nums);
Console.WriteLine($"{t} [{string.Join(", ", nums)}]");

static int Total(List<int> xs)
{
    var sum = 0;
    foreach (var x in xs)
    {
        sum += x;
    }
    return sum;
}
