var xs = new List<int> { 1, 2, 3 };
try
{
    foreach (var x in xs)
        xs.Add(x * 10);
}
catch (InvalidOperationException e)
{
    Console.WriteLine($"caught: {e.Message}");
}
Console.WriteLine(string.Join(", ", xs));
