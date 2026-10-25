Console.WriteLine(MakeGreeting("rust"));

static string MakeGreeting(string name)
{
    var s = $"hello {name}";
    return s;
}
