var d = new Dictionary<string, int> { ["one"] = 1, ["two"] = 2, ["three"] = 3 };
d.Remove("one");
d["four"] = 4;
Console.WriteLine("Dictionary: " + string.Join(", ", d.Keys));
Console.WriteLine("SortedDictionary: " + string.Join(", ", new SortedDictionary<string, int>(d).Keys));

try
{
    d[null!] = 0;
}
catch (ArgumentNullException)
{
    Console.WriteLine("null key: ArgumentNullException");
}
