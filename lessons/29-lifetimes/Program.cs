// Nothing to annotate: the garbage collector keeps a string alive while it is reachable.
static string Longest(string a, string b) => a.Length >= b.Length ? a : b;

string novel = "Call me Ishmael. Some years ago...";
string first = novel.Substring(0, novel.IndexOf('.'));
var excerpt = new Excerpt(first);
string l = Longest(excerpt.Part, "short");
Console.WriteLine($"longest: {l}");
Console.WriteLine($"excerpt length: {excerpt.Part.Length}");

record Excerpt(string Part);
