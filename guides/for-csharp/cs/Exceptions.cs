static int Double(string text) => int.Parse(text) * 2;   // nothing here says it can throw

Console.WriteLine(Double("21"));
try
{
    Console.WriteLine(Double("x"));
}
catch (FormatException e)
{
    Console.WriteLine($"caught: {e.Message}");
}
