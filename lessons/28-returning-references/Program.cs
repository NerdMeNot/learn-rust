var g = MakeGreeting("rust");
var w = FirstWord(g);
Console.WriteLine($"{g} / {w}");

// returns a NEW string
static string MakeGreeting(string name) => $"hello {name}";

// returns a view into the input: no copy
static ReadOnlySpan<char> FirstWord(ReadOnlySpan<char> text)
{
    var space = text.IndexOf(' ');
    return space < 0 ? text : text[..space];
}
