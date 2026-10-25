Console.WriteLine(Largest([3, 9, 2]));
Console.WriteLine(Largest(["kiwi", "apple", "pear"]));

// T can be any type that can compare itself to another T
static T Largest<T>(T[] items) where T : IComparable<T>
{
    var best = items[0];
    foreach (var item in items)
    {
        if (item.CompareTo(best) > 0)
        {
            best = item;
        }
    }
    return best;
}
