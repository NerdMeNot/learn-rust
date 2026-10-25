static string Longest(string a, string b) => a.Length >= b.Length ? a : b;

// Join builds a new string at run time: a heap object, not a literal.
static string Join(params string[] parts) => string.Join(" ", parts);

string s1 = Join("a", "long", "first", "string");
string result, other;
{
    string s2 = Join("short", "string");
    result = Longest(s1, s2);
    string s3 = Join("an", "even", "longer", "second", "string");
    other = Longest(s1, s3);
}
Console.WriteLine($"longest: {result}");
Console.WriteLine($"with a longer s3: {other}");
