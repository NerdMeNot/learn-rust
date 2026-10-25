var counts = new Dictionary<string, int>();
counts["cat"] = 0;
foreach (var w in "cat cat".Split(' '))
{
    counts[w] += 1;
}
Console.WriteLine(string.Join(", ", counts));
