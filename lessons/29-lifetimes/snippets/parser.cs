var parser = new Parser("lifetimes are regions");
string? a = parser.NextWord();
string? b = parser.NextWord(); // a is still usable: no annotation needed
Console.WriteLine($"a: {a}");
Console.WriteLine($"b: {b}");
string? again = new Parser("lifetimes are regions").NextWord();
Console.WriteLine($"same object as a: {ReferenceEquals(a, again)}");

class Parser(string input)
{
    private string rest = input;

    public string? NextWord()
    {
        string trimmed = rest.TrimStart();
        if (trimmed.Length == 0) return null;
        int space = trimmed.IndexOf(' ');
        rest = space < 0 ? "" : trimmed.Substring(space + 1);
        return space < 0 ? trimmed : trimmed.Substring(0, space);
    }
}
