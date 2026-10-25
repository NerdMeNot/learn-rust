foreach (var input in new[] { "42", "abc" })
{
    try
    {
        Console.WriteLine($"age {ParseAge(input)}");
    }
    catch (FormatException e)
    {
        Console.WriteLine($"error: {e.Message}");
    }
}

// a bad string throws, and the exception flies up to the caller
static uint ParseAge(string s) => uint.Parse(s);
