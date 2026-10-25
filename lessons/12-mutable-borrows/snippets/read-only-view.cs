var list = new List<int> { 1, 2, 3 };
IReadOnlyList<int> view = list; // no Add on this type: read-only through `view`

list.Add(4); // the other reference can still change it
Console.WriteLine($"view after list.Add(4): [{string.Join(", ", view)}]");

var items = new List<int> { 1, 2, 3, 4 };
try
{
    foreach (var x in items)
    {
        if (x == 3) items.Remove(3); // the second-to-last item
    }
}
catch (InvalidOperationException e)
{
    Console.WriteLine($"remove in a foreach: {e.Message}");
}
