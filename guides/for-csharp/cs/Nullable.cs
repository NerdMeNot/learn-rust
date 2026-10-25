string? Find(string key) => key == "a" ? "apple" : null;

var s = Find("b");
try
{
    Console.WriteLine(s.Length);
}
catch (NullReferenceException e)
{
    Console.WriteLine($"caught {e.GetType().Name}");
}
